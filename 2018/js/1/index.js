import fs from 'fs'

function main() {
  const data = fs.readFileSync(process.argv[2]).toString('utf8')
  const lines = data.trim().split('\n')

  const history = new Set()
  let value = 0
  let found = false
  while (!found) {
    for (const line of lines) {
      const line_value = parseInt(line, 10)
      value += line_value

      if (history.has(value)) {
        found = true
        break
      }
      history.add(value)
    }
  }

  console.log(value)
}

main()
