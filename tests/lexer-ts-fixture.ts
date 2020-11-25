type A = "x" | "y" | "z"

type B = {
    b: 'b'
}

type C = {
    c: 'c'
}
type X<T> = T extends A ? "Good" : "Bad";

const returnsGood = (): X<"x"> => "Good";
const returnsBad = (): X<"j"> => "Bad";

type fn = () => Promise<unknown>;

const asynFn: fn = () => new Promise<number>(() => {});

const intersection = () : B&C => ({
    b: 'b',
    c : "c",
});