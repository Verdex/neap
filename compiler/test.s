
mod x.y.z {
    use a.b.c;

    type a = type_info;
    type b = c;
    type d<a> = Con(a, d<a>) | Nil;
    type j = { a : b, c : d };
    type k = Con { a : b, c : d } | Nil;

    sig w {

        type a;
        let z : type_info = expr; 
        fun x ( a : ta, b : tb, ... ) -> xt;
        ext y ( a : ta, b : tb, ... ) -> yt;
    }
}

mod blah<a> {
    use blarg<b>;
    use zap<c> as h;

    fun x<b>( a : fun<b, c> ) -> b {
        h.some_function(1, 2, 3);
        return [1, 2, 3].map( < %1 + 1 > );
    }

}
