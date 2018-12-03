import fs from 'fs'

function main() {
  const data = fs.readFileSync(process.argv[2]).toString('utf8')
  const lines = data.trim().split('\n')

  const addCount = (counts, key) => {
    let value = counts[key]
    if (value === undefined) {
      value = 0
    }
    counts[key] = value + 1
  }

  let doubles = 0
  let triples = 0
  for (const line of lines) {
    const counts = {}
    for (const char of line) {
      addCount(counts, char)
    }

    let hasDouble = false
    let hasTriple = false
    for (const key in counts) {
      if (counts[key] === 2) {
        hasDouble = true
      }
      if (counts[key] === 3) {
        hasTriple = true
      }
    }

    if (hasDouble) {
      doubles += 1
    }
    if (hasTriple) {
      triples += 1
    }
  }

  console.log(doubles * triples)
}

main()
