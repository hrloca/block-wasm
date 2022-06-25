use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local!(static TASK_ID: Rc<RefCell<u64>> = Rc::new(RefCell::new(0)));

type Task<T> = Box<dyn Fn(&crate::Ctx) -> Option<T>>;

pub fn issue_task_id() -> Rc<RefCell<u64>> {
    TASK_ID.with(|rc| *rc.borrow_mut() += 1);
    TASK_ID.with(|rc| rc.clone())
}

pub struct TaskScheduler<T> {
    tasks: HashMap<u64, Task<T>>,
    relation: HashMap<u64, u64>,
    cb: Box<dyn Fn(u64, T)>,
}

impl<T> TaskScheduler<T> {
    pub fn create(cb: Box<dyn Fn(u64, T)>) -> Self {
        TaskScheduler {
            tasks: HashMap::new(),
            relation: HashMap::new(),
            cb,
        }
    }

    pub fn then(&mut self, prev_task_id: u64, task: Task<T>) -> u64 {
        let task_id = self.register(task);
        self.relation.insert(prev_task_id, task_id);
        task_id
    }

    pub fn register(&mut self, task: Task<T>) -> u64 {
        let task_id = *issue_task_id().borrow_mut();
        self.tasks.insert(task_id, task);
        task_id
    }

    pub fn jump(&mut self, from: u64, to: u64) {
        self.relation.insert(from, to);
    }

    pub fn run(&mut self, task_id: u64) -> u64 {
        let v_task_id = *issue_task_id().borrow_mut();
        self.relation.insert(v_task_id, task_id);
        v_task_id
    }

    pub fn exec(&self, completed: &Vec<u64>, ctx: &crate::Ctx) {
        for comp_id in completed.iter() {
            if let Some(next_task_id) = self.relation.get(&comp_id) {
                if let Some(next_task) = self.tasks.get(&next_task_id) {
                    if let Some(task) = next_task(&ctx) {
                        (self.cb)(*next_task_id, task);
                    }
                }
            }
            ctx.action_dispacher.delete_complete(*comp_id);
        }
    }
}
