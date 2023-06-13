///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    New,
    Ready,
    Running,
    Blocked,
    Zombie,
    Terminated,
}

/// Struct of the node
///
/// ```
/// # use filasse::node::*;
/// struct node {
///     state: State,
///     priority: u32,
///     duration: u64,
/// }
#[derive(Debug, Copy, Clone)]
pub struct Node {
    state: State,
    priority: u32,
    duration: u64,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            state: State::New,
            priority: 0,
            duration: 0,
        }
    }
}

impl Node {
    /// Creates a new Node
    ///
    /// Takes two arguments which are the priority and the duration. The methode will give you back a node with a `State::New` and the values given. It alse exist the default method to create the node. With it, no argument are require and the value of priority and duration will be set to Zero.
    /// # Examples
    /// Example 1 :
    ///```
    /// # use filasse::node::*;
    /// let node = Node::new(1,2);
    ///```
    /// Example 2 :
    ///
    ///```
    /// # use filasse::node::*;
    /// let node = Node::default();
    ///```
    pub fn new(priority: u32, duration: u64) -> Self {
        Node {
            state: State::New,
            priority,
            duration,
        }
    }

    /// Get state value
    ///
    /// This method is used to get the value of the node's state
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let node = Node::default();
    /// let status = node.state();
    /// ```
    pub fn state(&self) -> State {
        self.state
    }

    /// Get priority value
    ///
    /// This method is used to get the value of the node's priority
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let node = Node::default();
    /// let status = node.priority();
    /// ```
    pub fn priority(&self) -> u32 {
        self.priority
    }

    /// Get duration value
    ///
    /// This method is used to get the value of the node's duration
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let node = Node::default();
    /// let status = node.duration();
    /// ```
    pub fn duration(&self) -> u64 {
        self.duration
    }

    /// Set state value
    ///
    /// This method is used to set the value of the node's state. It takes a state in arguments. The node must alse to be mutable.
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default();
    /// let status = node.set_state(State::Running);
    /// ```
    pub fn set_state(&mut self, value: State) {
        self.state = value;
    }

    /// Set priority value
    ///
    /// This method is used to set the value of the node's priority  It takes a priority which is a u32 type in arguments. The node must alse to be mutable.
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default();
    /// let status = node.set_priority(2);
    /// ```
    pub fn set_priority(&mut self, value: u32) {
        self.priority = value;
    }

    /// Set duration value
    ///
    /// This method is used to set the value of the node's duration  It takes a duration which is a u64 type in arguments. The node must alse to be mutable.
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default();
    /// let status = node.set_duration(2);
    /// ```
    pub fn set_duration(&mut self, value: u64) {
        self.duration = value;
    }

    /// Change node to ready
    ///
    /// This method allows the node to pass the Ready state. To become ready, the node must be in the New or Running state. The node must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default(); // node = {status: State::New, priority: 0, duration: 0 }
    /// node.ready();
    /// ```
    pub fn ready(&mut self) {
        self.state = match self.state {
            State::New => State::Ready,
            State::Running => State::Ready,
            _ => self.state.clone(),
        }
    }

    /// Lock the node
    ///
    /// This method allows the node to pass the Blocked state. To become Blocked, the node must be in the Running state. The node must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default(); // node = {status: State::New, priority: 0, duration: 0 }
    /// node.set_state(State::Running);
    /// node.lock();
    /// ```
    pub fn lock(&mut self) {
        self.state = match self.state {
            State::Running => State::Blocked,
            _ => self.state.clone(),
        }
    }

    /// Unlock the node
    ///
    /// This method allows the node to return in the Ready state. To be unlocked, the node must be in the Locked state. The node must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default(); // node = {status: State::New, priority: 0, duration: 0 }
    /// node.set_state(State::Blocked);
    /// node.unlock();
    /// ```
    pub fn unlock(&mut self) {
        self.state = match self.state {
            State::Blocked => State::Ready,
            _ => self.state.clone(),
        }
    }

    /// Run the node
    ///
    /// This method allows the node to run and pass the Running state. To run, the node must be in the Ready state. The node must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default(); // node = {status: State::New, priority: 0, duration: 0 }
    /// node.set_state(State::Ready);
    /// node.run();
    /// ```
    pub fn run(&mut self) {
        self.state = match self.state {
            State::Ready => State::Running,
            _ => self.state.clone(),
        }
    }

    /// Change node to Zombie
    ///
    /// This method allows the node to pass the Zombie state. To become Zombie, the node must be in the Running state. The node must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default(); // node = {status: State::New, priority: 0, duration: 0 }
    /// node.set_state(State::Running);
    /// node.zombie();
    /// ```
    pub fn zombie(&mut self) {
        self.state = match self.state {
            State::Running => State::Zombie,
            State::Blocked => State::Zombie,
            _ => self.state.clone(),
        }
    }

    /// Unschedule the node
    ///
    /// This method allows the node to be unschedule after its job. To become Terminated, the node must be in the Zombie state. The node must alse to be mutable.
    ///
    /// # Example :
    /// ```
    /// # use filasse::node::*;
    /// let mut node = Node::default(); // node = {status: State::New, priority: 0, duration: 0 }
    /// node.set_state(State::Zombie);
    /// node.finish();
    /// ```
    pub fn finish(&mut self) {
        self.state = match self.state {
            State::Zombie => State::Terminated,
            _ => self.state.clone(),
        }
    }
}
