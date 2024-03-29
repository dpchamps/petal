type X10<T> = T extends (infer U extends number ? 1 : 0) ? 1 : 0; // ok, parsed as conditional
type X11<T> = T extends ((infer U) extends number ? 1 : 0) ? 1 : 0; // ok, parsed as conditional
type X12<T> = T extends (infer U extends number) ? 1 : 0; // ok, parsed as `infer..extends` (no trailing `?`)
type X13<T> = T extends infer U extends number ? 1 : 0; // ok, parsed as `infer..extends` (conditional types not allowed in 'extends type')
type X14<T> = T extends keyof infer U extends number ? 1 : 0; // ok, parsed as `infer..extends` (precedence wouldn't have parsed the `?` as part of a type operator)
type X15<T> = T extends { [P in infer U extends keyof T ? 1 : 0]: 1; } ? 1 : 0; // ok, parsed as conditional
type X16<T> = T extends { [P in infer U extends keyof T]: 1; } ? 1 : 0; // ok, parsed as `infer..extends` (no trailing `?`)
type X17<T> = T extends { [P in keyof T as infer U extends P ? 1 : 0]: 1; } ? 1 : 0; // ok, parsed as conditional
type X18<T> = T extends { [P in keyof T as infer U extends P]: 1; } ? 1 : 0; // ok, parsed as `infer..extends` (no trailing `?`)
