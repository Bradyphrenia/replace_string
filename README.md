this is a postgresql modul written in rust using regex crate for replacing parts of a string

usage:

create extension replace_string;

select replace_string('h@llo', '@', 'e'); -- h@llo -> hello

