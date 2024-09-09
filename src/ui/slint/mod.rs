use std::{rc::Rc, sync::RwLock};

use slint::{Model, ModelRc, PlatformError, SharedString, VecModel};

use crate::model::{
    self,
    filters::Filter,
    workspaces::{HierarchyElement, Workspace},
};

slint::include_modules!();

pub struct SlintUI {
    model: Rc<RwLock<model::Model>>,
    ui: AppWindow,
}

impl SlintUI {
    pub fn new(model: model::Model) -> Result<SlintUI, slint::PlatformError> {
        let ui = AppWindow::new()?;
        let slint_ui = SlintUI {
            model: Rc::new(RwLock::new(model)),
            ui,
        };
        slint_ui.init_data()?;
        slint_ui.register_callbacks();
        Ok(slint_ui)
    }

    fn init_data(&self) -> Result<(), slint::PlatformError> {
        let model_read = self
            .model
            .read()
            .expect("Model should be readable during initialization");

        let current_workspace = model_read
            .workspaces
            .get(0)
            .ok_or(PlatformError::Other("Test".to_string()))?;
        let current_workspace_name = current_workspace.name.as_str();

        self.ui
            .set_workspaces(to_workspace_model(&model_read.workspaces));
        self.ui
            .set_current_workspace(SharedString::from(current_workspace_name));
        self.ui
            .set_generation_entries(to_entries_model(&current_workspace.hierarchy));

        self.ui.set_filters(to_ui_filters(&model_read.filter_list));

        Ok(())
    }

    fn register_callbacks(&self) {
        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui
            .on_generators_entry_selected(move |current_workspace, id, is_folder| {
                if is_folder {
                    let model = model_clone
                        .upgrade()
                        .expect("Model should not be dropped before the end of the program");
                    {
                        model
                            .write()
                            .expect("Model is not writable, but a menu need to be fold")
                            .reverse_folding(&current_workspace, &id)
                            .expect("Could not fold/unfold folder");
                    }
                    let model_read = model.read().expect("Model should be readable");
                    ui_clone
                        .upgrade()
                        .expect("UI should not be dropped before the end of the program")
                        .set_generation_entries(to_entries_model(
                            &model_read
                                .get_current_workspace()
                                .expect("Current workspace not found - should not happen")
                                .hierarchy,
                        ))
                } else {
                    let ui = ui_clone
                    .upgrade()
                    .expect("UI should not be dropped before the end of the program");
                    let tabs_rc = ui
                        .get_tabs();
                    let tabs = tabs_rc
                        .as_any()
                        .downcast_ref::<VecModel<TabData>>()
                        .expect("We know we set a VecModel earlier");
                    let found_tabs: Vec<(usize, TabData)> = tabs.iter().enumerate().filter(|(_, t)| t.workspace_name == current_workspace && t.id == &id).collect();
                    let mut active_tab = tabs.row_count();
                    if found_tabs.len() == 0 {
                        let path = model::id_to_path(&id);
                        tabs.push(TabData {
                            workspace_name: current_workspace.to_string().into(),
                            id: id.to_string().into(),
                            name: path[path.len() - 1].to_string().into(),
                            content: id + "

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris dapibus aliquam condimentum. Vivamus aliquet mauris ligula, eget lobortis dui congue vitae. Curabitur ligula nisl, aliquam eget lorem id, semper sagittis purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec pulvinar risus rhoncus risus gravida suscipit nec eget velit. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus eu interdum magna. Donec efficitur lectus leo, eu scelerisque mi congue at. Sed vitae elit est. Aenean nec magna quis dui ultricies scelerisque. Donec ullamcorper, arcu id fringilla sagittis, dui ex volutpat neque, eu aliquam eros ex nec sem. Phasellus ante mi, finibus efficitur lacinia vitae, lobortis quis arcu.

Donec sit amet dui lacus. Fusce semper interdum viverra. Nulla et sodales augue. Nulla in dapibus velit, id ullamcorper mauris. Suspendisse potenti. Proin elit velit, venenatis suscipit mi at, auctor molestie erat. In hac habitasse platea dictumst. Nulla eget pretium sapien. Duis pellentesque sem sit amet ligula tristique blandit. Curabitur ac diam semper, consequat odio vitae, consequat quam. Donec aliquet dignissim ligula, suscipit sodales sapien pellentesque nec. Nam ultrices ligula est, in fermentum nisi condimentum et. Aenean lacinia, risus nec vulputate pharetra, leo nibh sodales tellus, ac tincidunt nulla risus eget lacus.

Vestibulum a mattis augue. Pellentesque sit amet vulputate ipsum, quis iaculis elit. Sed efficitur, eros quis porttitor rhoncus, dui dolor ultrices turpis, eu tristique lectus augue a ligula. Ut est magna, bibendum et aliquet ut, porta ac mi. Vestibulum ut est at arcu feugiat placerat. Ut lacinia dui vitae ullamcorper dignissim. Morbi sit amet sollicitudin est, sit amet ultricies urna. Etiam lorem mauris, condimentum sit amet ornare quis, euismod ac ante. Morbi a elementum lacus, sit amet hendrerit leo. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque porta diam sit amet congue tristique. Nullam consequat odio turpis, vel cursus velit placerat at.

Morbi hendrerit nunc ut tincidunt dapibus. Duis id molestie ex. Phasellus at magna iaculis purus maximus sollicitudin. Nunc dictum sit amet dolor quis interdum. Nulla nibh nisi, rutrum id risus nec, tristique pulvinar arcu. Cras eget neque sodales, faucibus metus nec, accumsan augue. Vestibulum purus tortor, finibus non posuere scelerisque, scelerisque sed velit. Duis fringilla turpis magna, in efficitur tortor efficitur ut. Proin facilisis sodales scelerisque. Maecenas nibh eros, pharetra eu eros et, varius mattis augue. Praesent vulputate euismod arcu, lacinia mattis justo aliquet ac. Proin fringilla vestibulum purus, a hendrerit lorem commodo et. Pellentesque tristique diam quis aliquam eleifend. Nunc egestas diam tempus, cursus dui quis, pretium tellus.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris dapibus aliquam condimentum. Vivamus aliquet mauris ligula, eget lobortis dui congue vitae. Curabitur ligula nisl, aliquam eget lorem id, semper sagittis purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec pulvinar risus rhoncus risus gravida suscipit nec eget velit. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus eu interdum magna. Donec efficitur lectus leo, eu scelerisque mi congue at. Sed vitae elit est. Aenean nec magna quis dui ultricies scelerisque. Donec ullamcorper, arcu id fringilla sagittis, dui ex volutpat neque, eu aliquam eros ex nec sem. Phasellus ante mi, finibus efficitur lacinia vitae, lobortis quis arcu.

Donec sit amet dui lacus. Fusce semper interdum viverra. Nulla et sodales augue. Nulla in dapibus velit, id ullamcorper mauris. Suspendisse potenti. Proin elit velit, venenatis suscipit mi at, auctor molestie erat. In hac habitasse platea dictumst. Nulla eget pretium sapien. Duis pellentesque sem sit amet ligula tristique blandit. Curabitur ac diam semper, consequat odio vitae, consequat quam. Donec aliquet dignissim ligula, suscipit sodales sapien pellentesque nec. Nam ultrices ligula est, in fermentum nisi condimentum et. Aenean lacinia, risus nec vulputate pharetra, leo nibh sodales tellus, ac tincidunt nulla risus eget lacus.

Vestibulum a mattis augue. Pellentesque sit amet vulputate ipsum, quis iaculis elit. Sed efficitur, eros quis porttitor rhoncus, dui dolor ultrices turpis, eu tristique lectus augue a ligula. Ut est magna, bibendum et aliquet ut, porta ac mi. Vestibulum ut est at arcu feugiat placerat. Ut lacinia dui vitae ullamcorper dignissim. Morbi sit amet sollicitudin est, sit amet ultricies urna. Etiam lorem mauris, condimentum sit amet ornare quis, euismod ac ante. Morbi a elementum lacus, sit amet hendrerit leo. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque porta diam sit amet congue tristique. Nullam consequat odio turpis, vel cursus velit placerat at.

Morbi hendrerit nunc ut tincidunt dapibus. Duis id molestie ex. Phasellus at magna iaculis purus maximus sollicitudin. Nunc dictum sit amet dolor quis interdum. Nulla nibh nisi, rutrum id risus nec, tristique pulvinar arcu. Cras eget neque sodales, faucibus metus nec, accumsan augue. Vestibulum purus tortor, finibus non posuere scelerisque, scelerisque sed velit. Duis fringilla turpis magna, in efficitur tortor efficitur ut. Proin facilisis sodales scelerisque. Maecenas nibh eros, pharetra eu eros et, varius mattis augue. Praesent vulputate euismod arcu, lacinia mattis justo aliquet ac. Proin fringilla vestibulum purus, a hendrerit lorem commodo et. Pellentesque tristique diam quis aliquam eleifend. Nunc egestas diam tempus, cursus dui quis, pretium tellus.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris dapibus aliquam condimentum. Vivamus aliquet mauris ligula, eget lobortis dui congue vitae. Curabitur ligula nisl, aliquam eget lorem id, semper sagittis purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec pulvinar risus rhoncus risus gravida suscipit nec eget velit. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus eu interdum magna. Donec efficitur lectus leo, eu scelerisque mi congue at. Sed vitae elit est. Aenean nec magna quis dui ultricies scelerisque. Donec ullamcorper, arcu id fringilla sagittis, dui ex volutpat neque, eu aliquam eros ex nec sem. Phasellus ante mi, finibus efficitur lacinia vitae, lobortis quis arcu.

Donec sit amet dui lacus. Fusce semper interdum viverra. Nulla et sodales augue. Nulla in dapibus velit, id ullamcorper mauris. Suspendisse potenti. Proin elit velit, venenatis suscipit mi at, auctor molestie erat. In hac habitasse platea dictumst. Nulla eget pretium sapien. Duis pellentesque sem sit amet ligula tristique blandit. Curabitur ac diam semper, consequat odio vitae, consequat quam. Donec aliquet dignissim ligula, suscipit sodales sapien pellentesque nec. Nam ultrices ligula est, in fermentum nisi condimentum et. Aenean lacinia, risus nec vulputate pharetra, leo nibh sodales tellus, ac tincidunt nulla risus eget lacus.

Vestibulum a mattis augue. Pellentesque sit amet vulputate ipsum, quis iaculis elit. Sed efficitur, eros quis porttitor rhoncus, dui dolor ultrices turpis, eu tristique lectus augue a ligula. Ut est magna, bibendum et aliquet ut, porta ac mi. Vestibulum ut est at arcu feugiat placerat. Ut lacinia dui vitae ullamcorper dignissim. Morbi sit amet sollicitudin est, sit amet ultricies urna. Etiam lorem mauris, condimentum sit amet ornare quis, euismod ac ante. Morbi a elementum lacus, sit amet hendrerit leo. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque porta diam sit amet congue tristique. Nullam consequat odio turpis, vel cursus velit placerat at.

Morbi hendrerit nunc ut tincidunt dapibus. Duis id molestie ex. Phasellus at magna iaculis purus maximus sollicitudin. Nunc dictum sit amet dolor quis interdum. Nulla nibh nisi, rutrum id risus nec, tristique pulvinar arcu. Cras eget neque sodales, faucibus metus nec, accumsan augue. Vestibulum purus tortor, finibus non posuere scelerisque, scelerisque sed velit. Duis fringilla turpis magna, in efficitur tortor efficitur ut. Proin facilisis sodales scelerisque. Maecenas nibh eros, pharetra eu eros et, varius mattis augue. Praesent vulputate euismod arcu, lacinia mattis justo aliquet ac. Proin fringilla vestibulum purus, a hendrerit lorem commodo et. Pellentesque tristique diam quis aliquam eleifend. Nunc egestas diam tempus, cursus dui quis, pretium tellus.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris dapibus aliquam condimentum. Vivamus aliquet mauris ligula, eget lobortis dui congue vitae. Curabitur ligula nisl, aliquam eget lorem id, semper sagittis purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec pulvinar risus rhoncus risus gravida suscipit nec eget velit. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus eu interdum magna. Donec efficitur lectus leo, eu scelerisque mi congue at. Sed vitae elit est. Aenean nec magna quis dui ultricies scelerisque. Donec ullamcorper, arcu id fringilla sagittis, dui ex volutpat neque, eu aliquam eros ex nec sem. Phasellus ante mi, finibus efficitur lacinia vitae, lobortis quis arcu.

Donec sit amet dui lacus. Fusce semper interdum viverra. Nulla et sodales augue. Nulla in dapibus velit, id ullamcorper mauris. Suspendisse potenti. Proin elit velit, venenatis suscipit mi at, auctor molestie erat. In hac habitasse platea dictumst. Nulla eget pretium sapien. Duis pellentesque sem sit amet ligula tristique blandit. Curabitur ac diam semper, consequat odio vitae, consequat quam. Donec aliquet dignissim ligula, suscipit sodales sapien pellentesque nec. Nam ultrices ligula est, in fermentum nisi condimentum et. Aenean lacinia, risus nec vulputate pharetra, leo nibh sodales tellus, ac tincidunt nulla risus eget lacus.

Vestibulum a mattis augue. Pellentesque sit amet vulputate ipsum, quis iaculis elit. Sed efficitur, eros quis porttitor rhoncus, dui dolor ultrices turpis, eu tristique lectus augue a ligula. Ut est magna, bibendum et aliquet ut, porta ac mi. Vestibulum ut est at arcu feugiat placerat. Ut lacinia dui vitae ullamcorper dignissim. Morbi sit amet sollicitudin est, sit amet ultricies urna. Etiam lorem mauris, condimentum sit amet ornare quis, euismod ac ante. Morbi a elementum lacus, sit amet hendrerit leo. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque porta diam sit amet congue tristique. Nullam consequat odio turpis, vel cursus velit placerat at.

Morbi hendrerit nunc ut tincidunt dapibus. Duis id molestie ex. Phasellus at magna iaculis purus maximus sollicitudin. Nunc dictum sit amet dolor quis interdum. Nulla nibh nisi, rutrum id risus nec, tristique pulvinar arcu. Cras eget neque sodales, faucibus metus nec, accumsan augue. Vestibulum purus tortor, finibus non posuere scelerisque, scelerisque sed velit. Duis fringilla turpis magna, in efficitur tortor efficitur ut. Proin facilisis sodales scelerisque. Maecenas nibh eros, pharetra eu eros et, varius mattis augue. Praesent vulputate euismod arcu, lacinia mattis justo aliquet ac. Proin fringilla vestibulum purus, a hendrerit lorem commodo et. Pellentesque tristique diam quis aliquam eleifend. Nunc egestas diam tempus, cursus dui quis, pretium tellus.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris dapibus aliquam condimentum. Vivamus aliquet mauris ligula, eget lobortis dui congue vitae. Curabitur ligula nisl, aliquam eget lorem id, semper sagittis purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec pulvinar risus rhoncus risus gravida suscipit nec eget velit. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus eu interdum magna. Donec efficitur lectus leo, eu scelerisque mi congue at. Sed vitae elit est. Aenean nec magna quis dui ultricies scelerisque. Donec ullamcorper, arcu id fringilla sagittis, dui ex volutpat neque, eu aliquam eros ex nec sem. Phasellus ante mi, finibus efficitur lacinia vitae, lobortis quis arcu.

Donec sit amet dui lacus. Fusce semper interdum viverra. Nulla et sodales augue. Nulla in dapibus velit, id ullamcorper mauris. Suspendisse potenti. Proin elit velit, venenatis suscipit mi at, auctor molestie erat. In hac habitasse platea dictumst. Nulla eget pretium sapien. Duis pellentesque sem sit amet ligula tristique blandit. Curabitur ac diam semper, consequat odio vitae, consequat quam. Donec aliquet dignissim ligula, suscipit sodales sapien pellentesque nec. Nam ultrices ligula est, in fermentum nisi condimentum et. Aenean lacinia, risus nec vulputate pharetra, leo nibh sodales tellus, ac tincidunt nulla risus eget lacus.

Vestibulum a mattis augue. Pellentesque sit amet vulputate ipsum, quis iaculis elit. Sed efficitur, eros quis porttitor rhoncus, dui dolor ultrices turpis, eu tristique lectus augue a ligula. Ut est magna, bibendum et aliquet ut, porta ac mi. Vestibulum ut est at arcu feugiat placerat. Ut lacinia dui vitae ullamcorper dignissim. Morbi sit amet sollicitudin est, sit amet ultricies urna. Etiam lorem mauris, condimentum sit amet ornare quis, euismod ac ante. Morbi a elementum lacus, sit amet hendrerit leo. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque porta diam sit amet congue tristique. Nullam consequat odio turpis, vel cursus velit placerat at.

Morbi hendrerit nunc ut tincidunt dapibus. Duis id molestie ex. Phasellus at magna iaculis purus maximus sollicitudin. Nunc dictum sit amet dolor quis interdum. Nulla nibh nisi, rutrum id risus nec, tristique pulvinar arcu. Cras eget neque sodales, faucibus metus nec, accumsan augue. Vestibulum purus tortor, finibus non posuere scelerisque, scelerisque sed velit. Duis fringilla turpis magna, in efficitur tortor efficitur ut. Proin facilisis sodales scelerisque. Maecenas nibh eros, pharetra eu eros et, varius mattis augue. Praesent vulputate euismod arcu, lacinia mattis justo aliquet ac. Proin fringilla vestibulum purus, a hendrerit lorem commodo et. Pellentesque tristique diam quis aliquam eleifend. Nunc egestas diam tempus, cursus dui quis, pretium tellus.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris dapibus aliquam condimentum. Vivamus aliquet mauris ligula, eget lobortis dui congue vitae. Curabitur ligula nisl, aliquam eget lorem id, semper sagittis purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Donec pulvinar risus rhoncus risus gravida suscipit nec eget velit. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Phasellus eu interdum magna. Donec efficitur lectus leo, eu scelerisque mi congue at. Sed vitae elit est. Aenean nec magna quis dui ultricies scelerisque. Donec ullamcorper, arcu id fringilla sagittis, dui ex volutpat neque, eu aliquam eros ex nec sem. Phasellus ante mi, finibus efficitur lacinia vitae, lobortis quis arcu.

Donec sit amet dui lacus. Fusce semper interdum viverra. Nulla et sodales augue. Nulla in dapibus velit, id ullamcorper mauris. Suspendisse potenti. Proin elit velit, venenatis suscipit mi at, auctor molestie erat. In hac habitasse platea dictumst. Nulla eget pretium sapien. Duis pellentesque sem sit amet ligula tristique blandit. Curabitur ac diam semper, consequat odio vitae, consequat quam. Donec aliquet dignissim ligula, suscipit sodales sapien pellentesque nec. Nam ultrices ligula est, in fermentum nisi condimentum et. Aenean lacinia, risus nec vulputate pharetra, leo nibh sodales tellus, ac tincidunt nulla risus eget lacus.

Vestibulum a mattis augue. Pellentesque sit amet vulputate ipsum, quis iaculis elit. Sed efficitur, eros quis porttitor rhoncus, dui dolor ultrices turpis, eu tristique lectus augue a ligula. Ut est magna, bibendum et aliquet ut, porta ac mi. Vestibulum ut est at arcu feugiat placerat. Ut lacinia dui vitae ullamcorper dignissim. Morbi sit amet sollicitudin est, sit amet ultricies urna. Etiam lorem mauris, condimentum sit amet ornare quis, euismod ac ante. Morbi a elementum lacus, sit amet hendrerit leo. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque porta diam sit amet congue tristique. Nullam consequat odio turpis, vel cursus velit placerat at.

Morbi hendrerit nunc ut tincidunt dapibus. Duis id molestie ex. Phasellus at magna iaculis purus maximus sollicitudin. Nunc dictum sit amet dolor quis interdum. Nulla nibh nisi, rutrum id risus nec, tristique pulvinar arcu. Cras eget neque sodales, faucibus metus nec, accumsan augue. Vestibulum purus tortor, finibus non posuere scelerisque, scelerisque sed velit. Duis fringilla turpis magna, in efficitur tortor efficitur ut. Proin facilisis sodales scelerisque. Maecenas nibh eros, pharetra eu eros et, varius mattis augue. Praesent vulputate euismod arcu, lacinia mattis justo aliquet ac. Proin fringilla vestibulum purus, a hendrerit lorem commodo et. Pellentesque tristique diam quis aliquam eleifend. Nunc egestas diam tempus, cursus dui quis, pretium tellus.

Fusce ut tortor nunc. Suspendisse ac quam molestie, lacinia enim et, tincidunt lorem. Proin tempus euismod tellus, ut consequat ex vestibulum eu. Proin eu ex pellentesque, finibus nisl id, auctor felis. Suspendisse vehicula pretium sem nec tempor. Nulla eu tempus nisl, sed rutrum tellus. Curabitur sagittis cursus odio. Integer vel eros dignissim, vulputate tortor ac, pellentesque augue. Morbi gravida at quam a euismod. Maecenas pretium ex at nisi facilisis congue. Cras eleifend justo sit amet rhoncus luctus. Quisque tellus leo, vestibulum et mauris vitae, mattis dignissim velit. Morbi quis mi mollis velit ultricies mollis. Fusce rutrum faucibus est, a dictum massa condimentum non.".into()
                        });
                    } else {
                        active_tab = found_tabs.get(0)
                        .expect("Found tabs should not be empty at this point - Checked ina previous condition")
                        .0;
                    }
                    ui.set_active_tab(active_tab.try_into()
                        .expect("Usize to i32 conversion should work"));
                }
            });

        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui.on_workspace_changed(move |workspace_name| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
            {
                model
                    .write()
                    .expect("Model is not writable, but a menu need to be fold")
                    .set_current_workspace(workspace_name.as_str());
            }
            let model_read = model.read().expect("Model should be readable");
            ui_clone
                .upgrade()
                .expect("UI should not be dropped before the end of the program")
                .set_generation_entries(to_entries_model(
                    &model_read
                        .get_current_workspace()
                        .expect("Current workspace not found - should not happen")
                        .hierarchy,
                ))
        });

        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui.on_filter_searched_tags(move |searched| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
            let model_read = model.read().expect("Model should be readable");
            let filtered_filters: Vec<FilterEntry> = model_read
                .filter_list
                .iter()
                .filter(|f| f.name.contains(searched.as_str()))
                .map(|f| FilterEntry {
                    name: SharedString::from(f.name.to_string()),
                    enable: f.enabled,
                })
                .collect();
            ui_clone
                .upgrade()
                .expect("UI should not be dropped before the end of the program")
                .set_filters(ModelRc::new(VecModel::from(filtered_filters)));
        });

        let model_clone = Rc::downgrade(&self.model);
        self.ui.on_reverse_filter_activation(move |filter_name| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
            {
                let mut model_write = model
                    .write()
                    .expect("Model is not writable, but a menu need to be fold");
                for filter in model_write.filter_list.iter_mut() {
                    if filter.name == filter_name.to_string() {
                        filter.enabled = !filter.enabled;
                    }
                }
            }
        });

        let model_clone = Rc::downgrade(&self.model);
        let ui_clone = self.ui.as_weak();
        self.ui.on_reset_filters(move |current_searched| {
            let model = model_clone
                .upgrade()
                .expect("Model should not be dropped before the end of the program");
            {
                let mut model_write = model
                    .write()
                    .expect("Model is not writable, but a menu need to be fold");
                for filter in model_write.filter_list.iter_mut() {
                    filter.enabled = false;
                }
            }
            println!("{}", current_searched);
            let model_read = model.read().expect("Model should be readable");
            let filtered_filters: Vec<FilterEntry> = model_read
                .filter_list
                .iter()
                .filter(|f| f.name.contains(current_searched.as_str()))
                .map(|f| FilterEntry {
                    name: SharedString::from(f.name.to_string()),
                    enable: f.enabled,
                })
                .collect();
            ui_clone
                .upgrade()
                .expect("UI should not be dropped before the end of the program")
                .set_filters(ModelRc::new(VecModel::from(filtered_filters)));
        });
        let ui_clone = self.ui.as_weak();
        self.ui.on_close_tab(move |data| {
            let ui = ui_clone
                    .upgrade()
                    .expect("UI should not be dropped before the end of the program");
            let tabs_rc = ui
                .get_tabs();
            let tabs = tabs_rc
                .as_any()
                .downcast_ref::<VecModel<TabData>>()
                .expect("We know we set a VecModel earlier");
            let found_tabs: Vec<(usize, TabData)> = tabs.iter().enumerate().filter(|(_, t)| t.workspace_name == data.workspace_name && t.id == &data.id).collect();
            if found_tabs.len() != 0 {
                let index = found_tabs.get(0)
                .expect("Found tabs should not be empty at this point - Checked ina previous condition")
                .0;
                tabs.remove(index);
            }
        });
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }
}

fn to_workspace_model(workspaces: &Vec<Workspace>) -> ModelRc<SharedString> {
    let tmp = workspaces
        .iter()
        .map(|s| SharedString::from(&s.name))
        .collect::<Vec<SharedString>>();
    let tmp = VecModel::from(tmp);
    ModelRc::new(tmp)
}

fn to_entries_model(entries: &Vec<HierarchyElement>) -> ModelRc<HierarchyEntry> {
    let mut hierarchy_entry = vec![];
    for element in entries {
        hierarchy_entry.extend(flatten_entry(element, 0, "/"))
    }

    let tmp = VecModel::from(hierarchy_entry);
    ModelRc::new(tmp)
}

fn to_ui_filters(filters: &Vec<Filter>) -> ModelRc<FilterEntry> {
    let filters_entry: Vec<FilterEntry> = filters
        .iter()
        .map(|f| FilterEntry {
            name: SharedString::from(f.name.to_string()),
            enable: f.enabled,
        })
        .collect();
    let filter_entries = VecModel::from(filters_entry);
    ModelRc::new(filter_entries)
}

fn flatten_entry(
    entry: &HierarchyElement,
    identation: i32,
    parent_id: &str,
) -> Vec<HierarchyEntry> {
    match entry {
        HierarchyElement::DashboardFolder(folder) => {
            let current_id = parent_id.to_string() + "/" + &folder.name;
            let mut elements = vec![HierarchyEntry {
                id: SharedString::from(parent_id.to_string() + "/" + &folder.name),
                title: SharedString::from(&folder.name),
                folded: false,
                identation,
                is_folder: true,
            }];

            if !folder.folded {
                for hierarchy_element in &folder.hierarchy {
                    elements.extend(flatten_entry(
                        hierarchy_element,
                        identation + 1,
                        &current_id,
                    ))
                }
            }
            return elements;
        }
        HierarchyElement::Dashboard(dashboard) => {
            return vec![HierarchyEntry {
                id: SharedString::from(parent_id.to_string() + "/" + &dashboard.name),
                title: SharedString::from(&dashboard.name),
                folded: false,
                identation,
                is_folder: false,
            }];
        }
    }
}
