#![cfg(feature = "invocation")]

extern crate error_chain;
extern crate jni;

use jni::objects::{AutoLocal, JObject};

mod util;
use util::{attach_current_thread, unwrap};

static ARRAYLIST_CLASS: &str = "java/util/ArrayList";
static EXCEPTION_CLASS: &str = "java/lang/Exception";
static ARITHMETIC_EXCEPTION_CLASS: &str = "java/lang/ArithmeticException";
static STRING_CLASS: &str = "java/lang/String";

#[test]
pub fn call_method_returning_null() {
    let env = attach_current_thread();
    // Create an Exception with no message
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(EXCEPTION_CLASS, "()V", &[])));
    // Call Throwable#getMessage must return null
    let message = unwrap(&env, env.call_method(obj.as_obj(), "getMessage", "()Ljava/lang/String;", &[]));
    let message_ref = env.auto_local(unwrap(&env, message.l()));

    assert!(message_ref.as_obj().is_null());
}

#[test]
pub fn is_instance_of_same_class() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(EXCEPTION_CLASS, "()V", &[])));
    assert!(unwrap(&env, env.is_instance_of(obj.as_obj(), EXCEPTION_CLASS)));
}

#[test]
pub fn is_instance_of_superclass() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(ARITHMETIC_EXCEPTION_CLASS, "()V", &[])));
    assert!(unwrap(&env, env.is_instance_of(obj.as_obj(), EXCEPTION_CLASS)));
}

#[test]
pub fn is_instance_of_subclass() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(EXCEPTION_CLASS, "()V", &[])));
    assert!(!unwrap(&env, env.is_instance_of(obj.as_obj(), ARITHMETIC_EXCEPTION_CLASS)));
}

#[test]
pub fn is_instance_of_not_superclass() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(ARITHMETIC_EXCEPTION_CLASS, "()V", &[])));
    assert!(!unwrap(&env, env.is_instance_of(obj.as_obj(), ARRAYLIST_CLASS)));
}

#[test]
pub fn is_instance_of_null() {
    let env = attach_current_thread();
    let obj = JObject::null();
    assert!(unwrap(&env, env.is_instance_of(obj, ARRAYLIST_CLASS)));
    assert!(unwrap(&env, env.is_instance_of(obj, EXCEPTION_CLASS)));
    assert!(unwrap(&env, env.is_instance_of(obj, ARITHMETIC_EXCEPTION_CLASS)));
}

#[test]
pub fn env_should_not_outlive_guard() {
    use jni::{JavaVM, JNIEnv};

    let vm: JavaVM;
    let env: JNIEnv;
    {
        let env_guard = attach_current_thread();
        vm = env_guard.get_java_vm().unwrap();
        // The lifetime of `env` is tied to the lifetime of `vm`, that is wrong
        env = vm.get_env().unwrap();
        assert!(vm.get_env().is_ok());
        let _ = unwrap(&env, env.get_version());
    }
    assert!(vm.get_env().is_err());
    // This line leads to SIGABRT
    // "FATAL ERROR in native method: Using JNIEnv in non-Java thread"
    let _ = unwrap(&env, env.get_version());
    println!("this println is unreachable");
}
