import eff from "./eff";

const secureRandom = async (count: number) => {
    // generate a cryptographically secure integer

    let cryptoObj = window.crypto;
    let rand = new Uint32Array(1);
    let skip = 0x7fffffff - (0x7fffffff % count);
    let result: number;

    if (((count - 1) & count) === 0) {
        cryptoObj.getRandomValues(rand);
        return rand[0] & (count - 1);
    }

    do {
        cryptoObj.getRandomValues(rand);
        result = rand[0] & 0x7fffffff;
    } while (result >= skip);

    return result % count;
};

export const getDicewareWords = async (length: number) => {
    // get the random words from the dice ware dict
    let wordslist = [];
    for (let i = 0; i < length; i++) {
        const newNum = [];
        for (let j = 0; j < 5; j += 1) {
            // roll a 6 sided die
            newNum.push((await secureRandom(6)) + 1);
        }
        let theWord = eff[Number(newNum.join(""))];
        wordslist.push(theWord);
    }

    return wordslist.join("-");
};

export const getHexPassword = async (length: number) => {
    const randomData = new Uint8Array(length);

    window.crypto.getRandomValues(randomData);
    
    // convert to hex
    return Array.from(randomData, (byte) => {
        return ("0" + (byte & 0xff).toString(16)).slice(-2);
    }).join("");
}
