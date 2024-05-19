const PARSED = [
    {
        query: 'A::B',
        elems: [{
            name: "a::b",
            fullPath: ["a", "b"],
            pathWithoutLast: ["a"],
            pathLast: "b",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "A::B",
        returned: [],
        userQuery: "a::b",
        error: null,
    },
    {
        query: "a:: a",
        elems: [{
            name: "a:: a",
            fullPath: ["a", "a"],
            pathWithoutLast: ["a"],
            pathLast: "a",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: 'a:: a',
        returned: [],
        userQuery: 'a:: a',
        error: null,
    },
    {
        query: "a ::a",
        elems: [{
            name: "a ::a",
            fullPath: ["a", "a"],
            pathWithoutLast: ["a"],
            pathLast: "a",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: 'a ::a',
        returned: [],
        userQuery: 'a ::a',
        error: null,
    },
    {
        query: "a  ::  a",
        elems: [{
            name: "a  ::  a",
            fullPath: ["a", "a"],
            pathWithoutLast: ["a"],
            pathLast: "a",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: 'a  ::  a',
        returned: [],
        userQuery: 'a  ::  a',
        error: null,
    },
    {
        query: 'A::B,C',
        elems: [
            {
                name: "a::b",
                fullPath: ["a", "b"],
                pathWithoutLast: ["a"],
                pathLast: "b",
                generics: [],
                typeFilter: -1,
            },
            {
                name: "c",
                fullPath: ["c"],
                pathWithoutLast: [],
                pathLast: "c",
                generics: [],
                typeFilter: -1,
            },
        ],
        foundElems: 2,
        original: 'A::B,C',
        returned: [],
        userQuery: 'a::b,c',
        error: null,
    },
    {
        query: 'A::B<f>,C',
        elems: [
            {
                name: "a::b",
                fullPath: ["a", "b"],
                pathWithoutLast: ["a"],
                pathLast: "b",
                generics: [
                    {
                        name: "f",
                        fullPath: ["f"],
                        pathWithoutLast: [],
                        pathLast: "f",
                        generics: [],
                    },
                ],
                typeFilter: -1,
            },
            {
                name: "c",
                fullPath: ["c"],
                pathWithoutLast: [],
                pathLast: "c",
                generics: [],
                typeFilter: -1,
            },
        ],
        foundElems: 2,
        original: 'A::B<f>,C',
        returned: [],
        userQuery: 'a::b<f>,c',
        error: null,
    },
    {
        query: 'mod::a',
        elems: [{
            name: "mod::a",
            fullPath: ["mod", "a"],
            pathWithoutLast: ["mod"],
            pathLast: "a",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "mod::a",
        returned: [],
        userQuery: "mod::a",
        error: null,
    },
];
