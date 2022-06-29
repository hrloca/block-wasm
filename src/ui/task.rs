use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

thread_local!(static TASK_ID: Rc<RefCell<u64>> = Rc::new(RefCell::new(0)));

type Task<T> = Box<dyn Fn(&crate::Ctx) -> Option<T>>;

pub fn issue_task_id() -> Rc<RefCell<u64>> {
    TASK_ID.with(|rc| *rc.borrow_mut() += 1);
    TASK_ID.with(|rc| rc.clone())
}

pub struct TaskScheduler<'a, T> {
    queue_pool: HashMap<&'a str, VecDeque<u64>>,
    tasks: HashMap<u64, Task<T>>,
    dequeue_event_listener: Box<dyn Fn(&str, T)>,
    end_queue_event_listener: Box<dyn Fn(&str)>,
}

impl<'a, T> TaskScheduler<'a, T> {
    pub fn create(
        dequeue_event_listener: Box<dyn Fn(&str, T)>,
        end_queue_event_listener: Box<dyn Fn(&str)>,
    ) -> Self {
        TaskScheduler {
            queue_pool: HashMap::new(),
            tasks: HashMap::new(),
            dequeue_event_listener,
            end_queue_event_listener,
        }
    }

    pub fn put(&mut self, name: &'a str, task: Task<T>) -> &'a str {
        let task_id = *issue_task_id().borrow_mut();
        match self.queue_pool.get_mut(name) {
            Some(tasks) => {
                tasks.push_back(task_id);
            }
            None => {
                self.queue_pool.insert(name, VecDeque::from([task_id]));
            }
        };
        self.tasks.insert(task_id, task);
        name
    }

    pub fn dequeue(&mut self, ctx: &crate::Ctx) {
        let next = &ctx.state.next_queue_task;
        for (name, queue) in self.queue_pool.iter_mut() {
            if !queue.is_empty() {
                if next.contains(&name.to_string()) {
                    // ctx.action_dispacher.active_queue_task(name.to_string());
                    // ctx.action_dispacher.delete_active_queue_task(name.to_string());
                } else {
                    if let Some(task_id) = queue.pop_front() {
                        if let Some(task) = self.tasks.get(&task_id) {
                            if let Some(content) = task(&ctx) {
                                (self.dequeue_event_listener)(name, content);
                                ctx.action_dispacher.next_queue_task(name.to_string());
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn exec(&mut self, ctx: &crate::Ctx) {
        self.dequeue(ctx);
    }
}
