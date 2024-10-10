
import {faker} from "@faker-js/faker";

import fs from "fs";

const list = [];
const totalItems = 100000;
for(let i = 0; i < totalItems; i++) {
    const height = 30+ Math.random() * 50;
    const _words = 10+ Math.random()*10
    list.push({
        index: i,
        defaultHeight:height,
        text: faker.word.words({count: _words})
    })
}

fs.writeFileSync("./data.json", JSON.stringify(list))
