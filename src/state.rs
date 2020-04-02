use std::rc::Rc;
use std::cell::RefCell;

pub type State = Rc<RefCell<FractalideState>>;

pub struct FractalideState {
    pub copernica: CopernicaState,
    pub luceo: LuceoState,
    pub whistle: WhistleState,
    pub settings: SettingsState,
    pub identities: IdentitiesState,
}

pub struct CopernicaState {
    pub query_val: String,
}

pub struct LuceoState {
    pub test_entry: String,
}

pub struct WhistleState {
    pub test_entry: String,
}

pub struct SettingsState {
    pub val: String,
}

pub struct IdentitiesState {
    pub current_identity: String,
}
