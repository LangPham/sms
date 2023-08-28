mod people;
pub use self::people::{People, PeopleInput};

mod brand;
pub use self::brand::{Brand, BrandInput};

mod campus;
pub use self::campus::{Campus, CampusInput};

mod custom_type;
pub use self::custom_type::*;

mod class;
pub use self::class::{Class, ClassInput};

mod student;
pub use self::student::{Student, StudentInput};

mod student_class;
pub use self::student_class::{StudentClass, StudentClassInput};
