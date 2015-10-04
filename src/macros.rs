#[macro_export]
macro_rules! Q_OBJECT_ft {
  (signal,$t:ty,$name:ident) => {
    impl $t {
      #[allow(dead_code)]
      fn $name (&self) {
        qmlrs::__qobject_emit(self,self.__rsqml_signal_id(stringify!($name)));
      }
    }
  };
  (slot,$t:ty,$name:ident) => {};
}

#[macro_export]
macro_rules! Q_OBJECT{
  (
    $t:ty:$(
      $ft:ident fn $name:ident($($at:ty),*);
    )*
  ) => (
    impl $t {
      #[allow(dead_code, unused_mut, unused_variables, unused_assignments)]
      fn __rsqml_signal_id(&self, name: &str) -> u32 {
        let mut id = 0;
        $(
          if stringify!($name) == name {
              return id;
          }
          id += 1;
        )+
        panic!("__rsqml_signal_id called with invalid signal name!");
      }
    }

    $(Q_OBJECT_ft!($ft,$t,$name))*

    ///MetaObject(rust类型)  含有一个c++对象QrsDynamicMetaObject MetaObject(rust类型)  含有一个c++对象QrsDynamicMetaObject 
    impl qmlrs::Object for $t {
      #[allow(unused_mut, unused_variables)]
      fn qt_metaobject(&self) -> qmlrs::MetaObject {
          let x = qmlrs::MetaObject::new();
          $(
            let mut argc = 0;
            $(
                let _: $at;
                argc += 1;
            )*
            let x = x.add_func(stringify!($ft),stringify!($name), argc);
          )+
          x
      }

      #[allow(unused_assignments, unused_mut, unused_variables)]
      fn qt_metacall(&mut self, slot: i32, args: *const *const qmlrs::OpaqueQVariant) {
        use qmlrs::ToQVariant;
        let mut i = 0;

        $(
          if i == slot {
              let mut argi = 1u8; /* 0 is for return value */
              let ret = self.$name(
                  $(
                      {
                          let _: $at;
                          match qmlrs::FromQVariant::from_qvariant(unsafe { *args.offset(argi as isize) }) {
                              Some(arg) => { argi += 1; arg }
                              None      => {
                                  println!("qmlrs: Invalid argument {} type for slot {}", argi, stringify!($name));
                                  return;
                              }
                          }
                      }
                  ),*
              );
              ret.to_qvariant(unsafe { *args.offset(0) as *mut qmlrs::OpaqueQVariant });
              return
          }
          i += 1;
        )+
      }
    }
  )
}