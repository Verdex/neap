
mod x::y::z {
    use a::b::c;

    struct a<b,c> {
        field1 : b;
        field2 : c;
        field3 : number;
    }

    union h<k, j> {
        option1 : k;
        option2 : j; 
        option3 : string;
    }

    struct z {
        y : union {
            a : number;
            b : string;
            c : struct { a : number; b : string };
        };
        x : number;
    }

    sig blah {
        abstract a;
        abstract b;
        
        fun x( self, one : a ) -> number;
        fun y<t>( self, one : a ); 

    }

    sig other<x, y> {

    }

    impl<x, y> other<x, y> for h<x, y> {

    }

    union blarg {
        a : b;
        c : d;
    }
    /* comment */
    // comment
    fun x<a, b>( input1 : a, input2 : b, input3 : string ) -> number {
        let h : number = 50.2;
        let z = "blah";

        set z = "other blah";
        fun w( a : number ) {

        }
        fun w2( a ) {

        }
        
        let y = <%1.add(%2)>;

        let l = [1,2,3];

        let x = { a : b, c : c };

        let www = blarg { a : 6 };

        case www {
            a => www.a;
            c => { www.c };
        }

        for x in y {

        }

        while true {

            if x {

                let z = try blah();

            }
            elseif y {

            }
            else {

                break;
                continue;
            }
        }
        return 0;
    }

    test name {

    }
}
