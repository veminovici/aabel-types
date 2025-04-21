pub struct Draft;
pub struct Published;
pub struct Archived;

pub struct Post<State> {
    content: String,
    _state: std::marker::PhantomData<State>,
}

// Operations available in draft state.
impl Post<Draft> {
    pub fn new(content: String) -> Post<Draft> {
        Post {
            content,
            _state: std::marker::PhantomData,
        }
    }

    pub fn edit(&mut self, content: String) {
        self.content = content;
    }

    pub fn publish(self) -> Post<Published> {
        Post {
            content: self.content,
            _state: std::marker::PhantomData,
        }
    }
}

// Operations available in published state.
impl Post<Published> {
    pub fn get_views(&self) -> u32 {
        // ...
        100
    }

    pub fn archive(self) -> Post<Archived> {
        Post {
            content: self.content,
            _state: std::marker::PhantomData,
        }
    }
}

// Operations available in archived state.
impl Post<Archived> {
    pub fn restore(self) -> Post<Published> {
        Post {
            content: self.content,
            _state: std::marker::PhantomData,
        }
    }
}
