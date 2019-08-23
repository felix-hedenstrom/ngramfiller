import target.debug.libngramconnector as lnc
import pprint
from enum import Enum
import time
class token(Enum):
    end = "!END"
timed_test_ngrams = None
try:
    import json
    with open("./optional/3grams.json", "r") as f:
        timed_test_ngrams = json.loads(f.read())
except:
    print("Did not load timed_test_ngrams")

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
                    },
                token.end:
                    {
                        token.end: 1
                    }

            },
        token.end:
            {
                    token.end:
                        {
                            token.end: 1    
                        }
            },
        "3gram":
            {
                "data":
                    {
                        "structure": 1,
                        token.end: 1
                    }
            }
    }

pprint.pprint(example_ngram) 
print(lnc.depth({"a": 1}))
print(lnc.depth({"a": {"b": 1}}))
print(lnc.depth(example_ngram))
print(lnc.bfs(example_ngram, 3, ["this"], ["structure"]))
print(lnc.bfs(example_ngram, 3, ["this"], [token.end]))
print(lnc.bfs(example_ngram, 3, ["non", "existing"], ["ngram"]))

if timed_test_ngrams:

    start = time.time()
    res = lnc.bfs(timed_test_ngrams, 3, ["i"], ["."])
    end = time.time()

    print("Search took {} seconds and resulted in \"{}\"".format(end - start, res))

