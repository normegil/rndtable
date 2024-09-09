use std::{rc::{Rc, Weak}, sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}};

use crate::model::{self, Model};

slint::include_modules!();

pub struct Controller {
    model: Weak<RwLock<model::Model>>,
    ui: i_slint_core::Weak<AppWindow>,
}

impl Controller {
    pub fn from(model: &Rc<RwLock<model::Model>>, ui: &AppWindow,) -> Controller {
        Controller {
            model: Rc::downgrade(&model),
            ui: ui.as_weak(),
        }
    }

    pub fn ui(&self) -> Rc<AppWindow> {
        let ui = self.ui
        .upgrade()
        .expect("UI should not be dropped before the end of the program");
        ui
    }

    pub fn model(&self) -> Rc<RwLock<model::Model>> {
        let model = self.model
        .upgrade()
        .expect("Model should not be dropped before the end of the program");
        model
    }

    pub fn model_read<'a>(&'a self) -> RwLockReadGuard<'a, Model> {
        let model = self.model()
        .read()
        .expect("Model should be readable (or wait for the lock to be released)");
        model
    }

    pub fn model_write<'a>(&'a self) -> RwLockWriteGuard<'a, Model> {
        let model = self.model()
        .write()
        .expect("Model should be writeable (or wait for the lock to be released)");
        model
    }
}