export function useNumberToText() {
  const arabicOnes = [
    "",
    "واحد",
    "اثنان",
    "ثلاثة",
    "أربعة",
    "خمسة",
    "ستة",
    "سبعة",
    "ثمانية",
    "تسعة",
    "عشرة",
    "أحد عشر",
    "اثنا عشر",
    "ثلاثة عشر",
    "أربعة عشر",
    "خمسة عشر",
    "ستة عشر",
    "سبعة عشر",
    "ثمانية عشر",
    "تسعة عشر",
  ];
  const arabicTens = [
    "",
    "عشرة",
    "عشرون",
    "ثلاثون",
    "أربعون",
    "خمسون",
    "ستون",
    "سبعون",
    "ثمانون",
    "تسعون",
  ];
  const arabicHundreds = [
    "",
    "مائة",
    "مئتان",
    "ثلاثمائة",
    "أربعمائة",
    "خمسمائة",
    "ستمائة",
    "سبعمائة",
    "ثمانمائة",
    "تسعمائة",
  ];
  const arabicScales = ["", "ألف", "مليون", "مليار", "تريليون"];

  const frenchOnes = [
    "",
    "un",
    "deux",
    "trois",
    "quatre",
    "cinq",
    "six",
    "sept",
    "huit",
    "neuf",
  ];
  const frenchTens = [
    "",
    "dix",
    "vingt",
    "trente",
    "quarante",
    "cinquante",
    "soixante",
    "soixante-dix",
    "quatre-vingt",
    "quatre-vingt-dix",
  ];
  const frenchScales = ["", "mille", "million", "milliard", "billion"];

  const englishOnes = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
  ];
  const englishTens = [
    "",
    "ten",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
  ];
  const englishScales = ["", "thousand", "million", "billion", "trillion"];

  const germanOnes = [
    "",
    "eins",
    "zwei",
    "drei",
    "vier",
    "fünf",
    "sechs",
    "sieben",
    "acht",
    "neun",
  ];
  const germanTens = [
    "",
    "zehn",
    "zwanzig",
    "dreißig",
    "vierzig",
    "fünfzig",
    "sechzig",
    "siebzig",
    "achtzig",
    "neunzig",
  ];
  const germanScales = ["", "tausend", "Million", "Milliarde", "Billion"];

  function convertToArabic(number: number): string {
    if (number === 0)
      return "صفر";
    if (number < 0)
      return `سالب ${convertToArabic(Math.abs(number))}`;

    let result = "";
    let scaleIndex = 0;

    while (number > 0) {
      const part = number % 1000;
      if (part > 0) {
        const partText = convertArabicPart(part);
        result
          = partText
          + (scaleIndex > 0 ? ` ${arabicScales[scaleIndex]} ` : "")
          + result;
      }
      number = Math.floor(number / 1000);
      scaleIndex++;
    }

    return result.trim();
  }

  function convertArabicPart(number: number): string {
    let result = "";
    const hundreds = Math.floor(number / 100);
    const tens = Math.floor((number % 100) / 10);
    const ones = number % 10;

    if (hundreds > 0) {
      result += `${arabicHundreds[hundreds]} `;
    }

    if (tens === 1) {
      result += `${arabicOnes[number % 100]} `;
    }
    else {
      if (tens > 1) {
        result += `${arabicTens[tens]} `;
      }
      if (ones > 0) {
        result += `${arabicOnes[ones]} `;
      }
    }

    return result.trim();
  }

  function convertToFrench(number: number): string {
    if (number === 0)
      return "zéro";
    if (number < 0)
      return `moins ${convertToFrench(Math.abs(number))}`;

    let result = "";
    let scaleIndex = 0;

    while (number > 0) {
      const part = number % 1000;
      if (part > 0) {
        const partText = convertFrenchPart(part);
        result
          = partText
          + (scaleIndex > 0 ? ` ${frenchScales[scaleIndex]} ` : "")
          + result;
      }
      number = Math.floor(number / 1000);
      scaleIndex++;
    }

    return result.trim();
  }

  function convertFrenchPart(number: number): string {
    let result = "";
    const hundreds = Math.floor(number / 100);
    const tens = Math.floor((number % 100) / 10);
    const ones = number % 10;

    if (hundreds > 0) {
      result += `${hundreds === 1 ? "cent" : `${frenchOnes[hundreds]} cent`} `;
    }

    if (tens === 1) {
      result += `${frenchTens[1]}-${frenchOnes[ones]} `;
    }
    else if (tens > 1) {
      result += frenchTens[tens];
      if (ones > 0) {
        result += `-${frenchOnes[ones]}`;
      }
      result += " ";
    }
    else if (ones > 0) {
      result += `${frenchOnes[ones]} `;
    }

    return result;
  }

  function convertToEnglish(number: number): string {
    if (number === 0)
      return "zero";
    if (number < 0)
      return `negative ${convertToEnglish(Math.abs(number))}`;

    let result = "";
    let scaleIndex = 0;

    while (number > 0) {
      const part = number % 1000;
      if (part > 0) {
        const partText = convertEnglishPart(part);
        result
          = partText
          + (scaleIndex > 0 ? ` ${englishScales[scaleIndex]} ` : "")
          + result;
      }
      number = Math.floor(number / 1000);
      scaleIndex++;
    }

    return result.trim();
  }

  function convertEnglishPart(number: number): string {
    let result = "";
    const hundreds = Math.floor(number / 100);
    const tens = Math.floor((number % 100) / 10);
    const ones = number % 10;

    if (hundreds > 0) {
      result += `${englishOnes[hundreds]} hundred `;
    }

    if (tens === 1) {
      result += `${
        [
          "ten",
          "eleven",
          "twelve",
          "thirteen",
          "fourteen",
          "fifteen",
          "sixteen",
          "seventeen",
          "eighteen",
          "nineteen",
        ][ones]
      } `;
    }
    else {
      if (tens > 1) {
        result += `${englishTens[tens]}-`;
      }
      if (ones > 0) {
        result += `${englishOnes[ones]} `;
      }
    }

    return result;
  }

  function convertToGerman(number: number): string {
    if (number === 0)
      return "null";
    if (number < 0)
      return `minus ${convertToGerman(Math.abs(number))}`;

    let result = "";
    let scaleIndex = 0;

    while (number > 0) {
      const part = number % 1000;
      if (part > 0) {
        const partText = convertGermanPart(part);
        result
          = partText
          + (scaleIndex > 0 ? ` ${germanScales[scaleIndex]} ` : "")
          + result;
      }
      number = Math.floor(number / 1000);
      scaleIndex++;
    }

    return result.trim();
  }

  function convertGermanPart(number: number): string {
    let result = "";
    const hundreds = Math.floor(number / 100);
    const tens = Math.floor((number % 100) / 10);
    const ones = number % 10;

    if (hundreds > 0) {
      result += `${germanOnes[hundreds]}hundert`;
    }

    if (tens === 1) {
      result += `${germanOnes[ones]}zehn`;
    }
    else {
      if (ones > 0) {
        result += germanOnes[ones];
      }
      if (tens > 1) {
        result += germanTens[tens];
      }
    }

    return result;
  }

  const numberToText = (number: number, language: string) => {
    const integerPart = Math.floor(number);
    const decimalPart = Math.round((number - integerPart) * 100);

    let result = "";

    switch (language.toLowerCase()) {
      case "ar":
        result = convertToArabic(integerPart);
        if (decimalPart > 0) {
          result += ` فاصلة ${convertToArabic(decimalPart)}`;
        }
        break;
      case "fr":
        result = convertToFrench(integerPart);
        if (decimalPart > 0) {
          result += ` virgule ${convertToFrench(decimalPart)}`;
        }
        break;
      case "en":
        result = convertToEnglish(integerPart);
        if (decimalPart > 0) {
          result += ` point ${convertToEnglish(decimalPart)}`;
        }
        break;
      case "de":
        result = convertToGerman(integerPart);
        if (decimalPart > 0) {
          result += ` Komma ${convertToGerman(decimalPart)}`;
        }
        break;
      default:
        result = "Unsupported language";
    }

    return result;
  };

  return {
    numberToText,
  };
}
