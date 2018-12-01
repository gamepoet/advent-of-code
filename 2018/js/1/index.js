import fs from 'fs'

function main() {
  const data = fs.readFileSync(process.argv[2]).toString('utf8')

  const lines = data.trim().split('\n')
  let value = 0
  for (const line of lines) {
    const line_value = parseInt(line, 10)

    value += line_value
  }

  console.log(value)
}

main()
