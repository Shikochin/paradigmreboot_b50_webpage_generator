// @ts-ignore
import fs from 'node:fs'
// @ts-ignore
import path from 'node:path'

const url =
    'https://paradigmrebootzh.miraheze.org/wiki/%E6%9B%B2%E7%9B%AE%E5%88%97%E8%A1%A8'

// @ts-ignore
const filePath = path.join(import.meta.dirname, 'wiki.html')

fetch(url)
    .then((response) => response.text())
    .then((data) => {
        fs.writeFileSync(filePath, data)
        console.log(`Wiki HTML saved to ${filePath}`)
    })
    .catch((error) => {
        console.error('Error fetching wiki HTML:', error)
    })
