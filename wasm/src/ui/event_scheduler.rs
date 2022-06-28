use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local!(static TASK_ID: Rc<RefCell<u64>> = Rc::new(RefCell::new(0)));

type Task<T> = Box<dyn Fn(&crate::Ctx) -> Option<T>>;

pub fn issue_task_id() -> Rc<RefCell<u64>> {
    TASK_ID.with(|rc| *rc.borrow_mut() += 1);
    TASK_ID.with(|rc| rc.clone())
}

pub struct EventScheduler<'a, T> {
    tasks: HashMap<&'a str, Task<T>>,
    relation: HashMap<&'a str, &'a str>,
    cb: Box<dyn Fn(&'a str, T)>,
}

impl<'a, T> EventScheduler<'a, T> {
    pub fn create(cb: Box<dyn Fn(&'a str, T)>) -> Self {
        EventScheduler {
            tasks: HashMap::new(),
            relation: HashMap::new(),
            cb,
        }
    }

    pub fn register(&mut self, name: &'a str, task: Task<T>) -> &'a str {
        self.tasks.insert(name, task);
        name
    }

    pub fn connect(&mut self, from: &'a str, to: &'a str) {
        self.relation.insert(from, to);
    }

    // TODO: delete task gabege.
    pub fn exec(&self, completed: &Vec<&'a str>, ctx: &crate::Ctx) {
        for comp_id in completed.iter() {
            if let Some(next_task_id) = self.relation.get(comp_id) {
                if let Some(next_task) = self.tasks.get(next_task_id) {
                    if let Some(task) = next_task(&ctx) {
                        (self.cb)(next_task_id, task);
                    }
                }
            }
        }
    }
}
