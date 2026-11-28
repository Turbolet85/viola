@row_set n=1 row=decision/unit-tests-inline cell=except seen=1 fact=F1 detector=none
value: A unit-test module too large for one file is split into sibling path files that keep its module path, its cases grouped into child modules, each helper standing once beside its only users.
basis: fct-y5ceijcr, fct-ssejqli4
because: The pump's and viola run's unit-test modules now live in #[path] files that keep their module paths, with cases in child modules and helpers beside their only users, so the inline rule must name this exception.
