type SupportedLanguage = "ar" | "fr" | "en";

export function useTotalAsText() {

  const arabicOnes = [
    "", "واحد", "اثنان", "ثلاثة", "أربعة", "خمسة", "ستة", "سبعة", "ثمانية", "تسعة",
    "عشرة", "أحد عشر", "اثنا عشر", "ثلاثة عشر", "أربعة عشر", "خمسة عشر", "ستة عشر", "سبعة عشر", "ثمانية عشر", "تسعة عشر",
  ];
  const arabicTens = ["", "عشرة", "عشرون", "ثلاثون", "أربعون", "خمسون", "ستون", "سبعون", "ثمانون", "تسعون"];
  const arabicHundreds = ["", "مائة", "مئتان", "ثلاثمائة", "أربعمائة", "خمسمائة", "ستمائة", "سبعمائة", "ثمانمائة", "تسعمائة"];
  const arabicScales = ["", "ألف", "مليون", "مليار", "تريليون"];


  const frenchOnes = ["", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf"];
  const frenchTeens = ["dix", "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf"];
  const frenchTens = ["", "dix", "vingt", "trente", "quarante", "cinquante", "soixante", "soixante-dix", "quatre-vingt", "quatre-vingt-dix"];
  const frenchScales = ["", "mille", "million", "milliard", "billion"];


  const englishOnes = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  const englishTeens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
  const englishTens = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
  const englishScales = ["", "thousand", "million", "billion", "trillion"];


  function convertToArabic(number: number): string {
    if (number === 0) return "صفر";
    if (number < 0) return `سالب ${convertToArabic(Math.abs(number))}`;

    let result: string[] = [];
    let scaleIndex = 0;

    while (number > 0) {
      const part = number % 1000;
      if (part > 0) {
        const partText = convertArabicPart(part);
        const scaleText = scaleIndex > 0 ? arabicScales[scaleIndex] : "";
        result.unshift(`${partText} ${scaleText}`.trim());
      }
      number = Math.floor(number / 1000);
      scaleIndex++;
    }

    return result.join(" و ").trim();
  }

  function convertArabicPart(number: number): string {
    const parts: string[] = [];
    const hundreds = Math.floor(number / 100);
    const remainder = number % 100;
    const tens = Math.floor(remainder / 10);
    const ones = remainder % 10;

    if (hundreds > 0) parts.push(arabicHundreds[hundreds]!);

    if (remainder > 0) {
      if (remainder < 20) {
        parts.push(arabicOnes[remainder]!);
      } else {
        if (ones > 0) parts.push(arabicOnes[ones]!);
        if (tens > 1) parts.push(arabicTens[tens]!);
      }
    }

    return parts.join(" و ");
  }


  function convertToFrench(number: number): string {
    if (number === 0) return "zéro";
    if (number < 0) return `moins ${convertToFrench(Math.abs(number))}`;

    let result = "";
    let scaleIndex = 0;

    while (number > 0) {
      const part = number % 1000;
      if (part > 0) {
        const partText = convertFrenchPart(part);
        const scale = scaleIndex > 0 ? frenchScales[scaleIndex] : "";

        const pluralScale = (part > 1 && scaleIndex > 1) ? `${scale}s` : scale;


        const isOneThousand = part === 1 && scaleIndex === 1;
        const prefix = isOneThousand ? "" : partText;

        result = `${prefix} ${pluralScale} ${result}`.trim();
      }
      number = Math.floor(number / 1000);
      scaleIndex++;
    }

    return result.replace(/\s+/g, ' ').trim();
  }

  function convertFrenchPart(number: number): string {
    let result = "";
    const hundreds = Math.floor(number / 100);
    const remainder = number % 100;

    if (hundreds > 0) {
      result += hundreds === 1 ? "cent " : `${frenchOnes[hundreds]} cent${remainder === 0 ? "s " : " "}`;
    }

    if (remainder > 0) {
      if (remainder < 10) {
        result += frenchOnes[remainder];
      } else if (remainder < 20) {
        result += frenchTeens[remainder - 10];
      } else {
        const tens = Math.floor(remainder / 10);
        const ones = remainder % 10;

        if (tens === 7 || tens === 9) {

          const baseTen = tens === 7 ? "soixante" : "quatre-vingt";
          const connector = (tens === 7 && ones === 1) ? " et " : "-";
          result += `${baseTen}${connector}${frenchTeens[ones]}`;
        } else {

          const is80Exact = tens === 8 && ones === 0;
          result += is80Exact ? "quatre-vingts" : frenchTens[tens];

          if (ones === 1 && tens !== 8) {
            result += ` et un`;
          } else if (ones > 1) {
            result += `-${frenchOnes[ones]}`;
          }
        }
      }
    }

    return result.trim();
  }


  function convertToEnglish(number: number): string {
    if (number === 0) return "zero";
    if (number < 0) return `negative ${convertToEnglish(Math.abs(number))}`;

    let result = "";
    let scaleIndex = 0;

    while (number > 0) {
      const part = number % 1000;
      if (part > 0) {
        const partText = convertEnglishPart(part);
        result = `${partText} ${scaleIndex > 0 ? englishScales[scaleIndex] : ""} ${result}`.trim();
      }
      number = Math.floor(number / 1000);
      scaleIndex++;
    }

    return result.replace(/\s+/g, ' ').trim();
  }

  function convertEnglishPart(number: number): string {
    let result = "";
    const hundreds = Math.floor(number / 100);
    const remainder = number % 100;

    if (hundreds > 0) {
      result += `${englishOnes[hundreds]} hundred `;
    }

    if (remainder > 0) {
      if (remainder < 10) {
        result += englishOnes[remainder];
      } else if (remainder < 20) {
        result += englishTeens[remainder - 10];
      } else {
        const tens = Math.floor(remainder / 10);
        const ones = remainder % 10;
        result += englishTens[tens];
        if (ones > 0) {
          result += `-${englishOnes[ones]}`;
        }
      }
    }

    return result.trim();
  }


  const numberToText = (number: number, language: SupportedLanguage | string) => {
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
      default:
        result = "Unsupported language";
    }

    return result;
  };

  return {
    numberToText,
  };
}
