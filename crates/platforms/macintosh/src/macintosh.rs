extern crate cocoa;
extern crate objc;

use cocoa::appkit::{NSApp, NSApplication, NSWindow};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use common::layer;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use tracing::{info, trace};

pub fn start() {
    std::thread::spawn(|| unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        let delegate = create_delegate();
        app.setDelegate_(delegate);

        app.run();
    });
}

fn create_delegate() -> id {
    unsafe {
        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("AppDelegate", superclass).unwrap();

        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            application_did_finish_launching as extern "C" fn(&Object, Sel, id),
        );

        decl.add_method(
            sel!(applicationDidActivate:),
            application_did_activate as extern "C" fn(&Object, Sel, id),
        );

        // decl.add_method(
        //     sel!(applicationDidDeactivate:),
        //     application_did_deactivate as extern "C" fn(&Object, Sel, id),
        // );

        let delegate_class = decl.register();
        let delegate: id = msg_send![delegate_class, new];
        delegate
    }
}

extern "C" fn application_did_finish_launching(_self: &Object, _cmd: Sel, _notification: id) {
    unsafe {
        let notification_center: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let notification_center: id = msg_send![notification_center, notificationCenter];

        let _: () = msg_send![notification_center, addObserver: _self
                          selector: sel!(applicationDidActivate:)
                              name: cocoa::foundation::NSString::alloc(nil).init_str("NSWorkspaceDidActivateApplicationNotification")
                            object: nil];

        // let _: () = msg_send![notification_center, addObserver: _self
        //                   selector: sel!(applicationDidDeactivate:)
        //                       name: cocoa::foundation::NSString::alloc(nil).init_str("NSWorkspaceDidDeactivateApplicationNotification")
        //                     object: nil];

        trace!("Application did finish launching");
    }
}

extern "C" fn application_did_activate(_self: &Object, _cmd: Sel, _notification: id) {
    unsafe {
        let user_info: id = msg_send![_notification, userInfo];
        let app: id = msg_send![user_info, objectForKey: cocoa::foundation::NSString::alloc(nil).init_str("NSWorkspaceApplicationKey")];

        let localized_name: id = msg_send![app, localizedName];
        let localized_name_str: *const i8 = msg_send![localized_name, UTF8String];

        let executable_url: id = msg_send![app, executableURL];
        let executable_path: id = msg_send![executable_url, path];
        let executable_path_str: *const i8 = msg_send![executable_path, UTF8String];

        // let bundle_id: id = msg_send![app, bundleIdentifier];
        // let bundle_id_str: *const i8 = msg_send![bundle_id, UTF8String];

        let app_details = common::structs::AppDetails {
            window: std::ffi::CStr::from_ptr(localized_name_str)
                .to_string_lossy()
                .to_string(),
            process: std::ffi::CStr::from_ptr(executable_path_str)
                .to_string_lossy()
                .to_string(),
        };

        info!("{:?}", &app_details);

        layer::process(&app_details);
    }
}

// extern "C" fn application_did_deactivate(_self: &Object, _cmd: Sel, _notification: id) {
//     trace!("Application did deactivate (unfocused)");
// }
