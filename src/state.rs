use std::rc::Rc;
use std::cell::RefCell;

pub type State = Rc<RefCell<FractalideState>>;

pub struct FractalideState {
    pub copernica: CopernicaState,
    pub luceo: LuceoState,
    pub settings: SettingsState,
}

pub struct CopernicaState {
    pub query_val: String,
}

pub struct LuceoState {
    pub query_val: String,
}

pub struct SettingsState {
    pub val: String,
}
