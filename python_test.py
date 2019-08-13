import target.debug.libngramconnector as lnc
import pprint

example_ngram = {
        "this":
            {
                "is":
                    {
                        "a": 1,
                        "one": 2,
                        "an": 3
                    },
                "should":
                    {
                        "work": 1
                    }
            },
        "is":
            {
                "an":
                    {
                        "example": 1
                    }
            },
        "an":
            {
                "example":
                    {
                        "of": 1    
                    }
            },
        "example":
            {
                "of":
                    {
                        "a": 1
                    }
            },
        "of":
            {
                "a":
                    {
                        "3gram": 1    
                    }
            },       
        "a":
            {
                "3gram":
                    {
                        "data": 1
                    }
            },
        "3gram":
            {
                "data":
                    {
                        "structure": 1
                    }
            }
    }

pprint.pprint(example_ngram) 
print(lnc.depth({"a": 1}))
print(lnc.depth({"a": {"b": 1}}))
print(lnc.depth(example_ngram))
print(lnc.bfs(example_ngram, 3, ["this"], ["structure"]))

