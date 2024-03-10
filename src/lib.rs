/*
An implementation of properties for Rust structs. The main goal here is to allow defining properties in Traits.
This is accomplished by adding methods to our traits that return our Property<TProp> type. This implementation
leads to some small memory overhead to create the Property wrapper, but leads to a convenient developer API for
defining the getter and setter along with composing the implementation of our getters and setters within the impl
of the struct itself.
*/


/*
TODO:
- "item" in the structs needs to be wrapped in some type of smart pointer to allow for shared references.
- finish having PropertyImplementation Into Property (along with the sole Getter and Setter types).
- unit tests
- add prototypes of "get_if()" and "set_if()" to allow for conditional assignment and access (in which case
  the get should return a Result)
- Observer pattern with a NotifyPropertyChanged Trait along with and Event dictionary and subscribe/unsubscribe
  methods.
*/

use std::cell::RefCell;
use std::rc::Rc;

/// Type used to get a value.
type GET<TProp> = fn() -> Rc<RefCell<TProp>>;
/// Type used to set a value.
type SET<TProp> = fn(TProp);

/// 
type GETTER<TStruct, TProp> = fn(this: TStruct) -> Rc<RefCell<TProp>>;
/// 
type SETTER<TStruct, TProp> = fn(this: TStruct, TProp);

/// Trait for getting a property value without knowing the owning item's type.
pub trait GetProp<TProp>
{
    fn get(&self) -> Rc<RefCell<TProp>>;
}

/// Trait for getting a property value with knowing the owning item's type.
pub trait GetProperty<TStruct, TProp>
{
    fn get(&self, item: TStruct) -> Rc<RefCell<TProp>>;
}

/// Trait for setting a property value without knowing the owning item's type.
pub trait SetProp<TProp> 
{
    fn set(&self, val: TProp);
}

/// Trait for setting a property value with knowing the owning item's type.
pub trait SetProperty<TStruct, TProp> 
{
    fn set(&self, item: TStruct, val: TProp);
}

/// Trait for implementing a property without knowing the owning item's type.
pub trait ImplProp<TProp> : GetProp<TProp> + SetProp<TProp> {}

/// Trait for implementing a property with knowing the owning item's type.
pub trait ImplProperty<TStruct, TProp> : GetProperty<TStruct, TProp> + GetProperty<TStruct, TProp> {}

/// Struct representing a property getter.
struct PropertyGetter<TStruct, TProp>
{
    item: TStruct,
    get_func: GETTER<TStruct, TProp>
}

/// Implementation of a property getter.
impl<TStruct, TProp> PropertyGetter<TStruct, TProp>
{
    pub fn new(item: TStruct, get_func: GETTER<TStruct, TProp>) -> Self 
    {
        Self 
        {
            item,
            get_func
        }
    }
}

/// Implementation of getting the property value from the owning item.
impl<TStruct, TProp> GetProperty<TStruct, TProp> for PropertyGetter<TStruct, TProp>
{
    fn get(&self, item: TStruct) -> Rc<RefCell<TProp>>
    {
        (self.get_func)(item)
    }
}

/// Struct representing a property setter.
struct PropertySetter<TStruct, TProp>
{
    item: TStruct,
    pub set_func: SETTER<TStruct, TProp>
}

/// Implementation of a property setter.
impl<TStruct, TProp> PropertySetter<TStruct, TProp>
{
    pub fn new(item: TStruct, set_func: SETTER<TStruct, TProp>) -> Self 
    {
        Self 
        {
            item,
            set_func
        }
    }
}

/// Implementation of setting the property value from the owning item.
impl<TStruct, TProp> SetProperty<TStruct, TProp> for PropertySetter<TStruct, TProp>
{
    fn set(&self, item: TStruct, val: TProp)
    {
        (self.set_func)(item, val);
    }
}

/// Struct representing a property implementing get and set behaviors from the owning item.
struct PropertyImplementation<TStruct, TProp>
{
    getter: PropertyGetter<TStruct, TProp>,
    setter: PropertySetter<TStruct, TProp>
}

/// 
impl<TStruct, TProp> Into<Property<TProp>> for PropertyImplementation<TStruct, TProp>
{
    fn into(self) -> Property<TProp> {
        todo!()
    }
}

///
impl<TStruct, TProp> PropertyImplementation<TStruct, TProp>
{
    pub fn new(item: TStruct, get_func: GETTER<TStruct, TProp>, set_func: SETTER<TStruct, TProp>) -> Self 
    {
        Self 
        {
            getter: PropertyGetter::new(item, get_func),
            setter: PropertySetter::new(item, set_func)
        }
    }
}

///
impl<TStruct, TProp> GetProperty<TStruct, TProp> for PropertyImplementation<TStruct, TProp>
{
    fn get(&self, item: TStruct) -> Rc<RefCell<TProp>> {
        self.getter.get(item)
    }
}

///
impl<TStruct, TProp> SetProperty<TStruct, TProp> for PropertyImplementation<TStruct, TProp>
{
    fn set(&self, item: TStruct, val: TProp) {
        self.setter.set(item, val)
    }
}

struct Property<TProp> 
{
    implementation: Box<dyn ImplProp<TProp>>
}

impl<TProp> Property<TProp>
{

}

impl<TProp> GetProp<TProp> for Property<TProp>
{
    fn get(&self) -> Rc<RefCell<TProp>> {
        todo!()
    }
}

//-------------------TEST-----------------------

#[cfg(test)]
mod tests {
    use super::*;

    trait HasSize 
    {
        fn size() -> Property<u64>;
    }

    struct Dog
    {
        _dog_size: Rc<RefCell<u64>>,
        dog_size: Property<u64>,
    }

    impl Dog
    {
        pub fn new(&self, size: u64) -> Self 
        {
            Self 
            {
                dog_size: PropertyImplementation::new
                (
                    self,
                    Dog::get_size,
                    Dog::set_size
                ).into(),
                _dog_size: Rc::new(RefCell::new(size))
            }
        }

        fn get_size(&self) -> Rc<RefCell<u64>> 
        {
            self._dog_size.to_owned()
        }

        fn set_size(&self, value: u64)
        {
            self._dog_size.replace(value);
        }

    }

    impl HasSize for Dog {
        fn size() -> Property<u64> {
            todo!()
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
