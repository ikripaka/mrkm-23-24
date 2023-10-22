## Особливості використання статичного класу **System.Numerics.BigInteger** в **C#**

1. Клас System.Numerics.BigInteger реалізує велику кількість різноманітних інтерфейсів, проте це не робить його використання складним, як, наприклад, в потребуючої більшої захищенності мові Rust.
2. Існує перевантаження операторів, що надає можливість більш природнього запису чисел, ніж, наприклад, в Java, в якій треба використовувати методи натомість.
3. Екземпляри класу в C# не можуть бути змінені під час використання, як от і в мові Java. Під час виконання операцій над числом буде створений новий екземпляр класу, бо числа будуть “immutable”, незмінні.
4. Використання цього класу в C# дає велику точність обчислень та можливість використовувати числа необмеженної довжини.
5. Великі числа можуть бути перетворені на числа інших типів.
6. Об’єкти цього класу можна серіалізувати.

## Детальніше про BigInteger

**BigInteger реалізує інтерфейси такі як:**
- IComparable<T>
- IComparable<BigInteger>  
- IEquatable<BigInteger>  
- IFormattable  
- ISpanFormattable  
- IComparable<TSelf>  
- IEquatable<TSelf>  
- IParsable<BigInteger>  
- IParsable<TSelf>  
- ISpanParsable<BigInteger>  
- ISpanParsable<TSelf>  
- IAdditionOperators<BigInteger,BigInteger,BigInteger>  
- IAdditionOperators<TSelf,TSelf,TSelf>  
- IAdditiveIdentity<BigInteger,BigInteger>  
- IAdditiveIdentity<TSelf,TSelf>  
- IBinaryInteger<BigInteger>  
- IBinaryNumber<BigInteger>  
- IBinaryNumber<TSelf>  
- IBitwiseOperators<BigInteger,BigInteger,BigInteger>  
- IBitwiseOperators<TSelf,TSelf,TSelf>  
- IComparisonOperators<BigInteger,BigInteger,Boolean>  
- IComparisonOperators<TSelf,TSelf,Boolean>  
- IDecrementOperators<BigInteger>  
- IDecrementOperators<TSelf>  
- IDivisionOperators<BigInteger,BigInteger,BigInteger>  
- IDivisionOperators<TSelf,TSelf,TSelf>  
- IEqualityOperators<BigInteger,BigInteger,Boolean>  
- IEqualityOperators<TSelf,TOther,TResult>  
- IEqualityOperators<TSelf,TSelf,Boolean>  
- IIncrementOperators<BigInteger>  
- IIncrementOperators<TSelf> 
- IModulusOperators<BigInteger,BigInteger,BigInteger> 
- IModulusOperators<TSelf,TSelf,TSelf> 
- IMultiplicativeIdentity<BigInteger,BigInteger> 
- IMultiplicativeIdentity<TSelf,TSelf> 
- IMultiplyOperators<BigInteger,BigInteger,BigInteger>  
- IMultiplyOperators<TSelf,TSelf,TSelf>  
- INumber<BigInteger>  
- INumber<TSelf>  
- INumberBase<BigInteger>  
- INumberBase<TSelf>  
- IShiftOperators<BigInteger,Int32,BigInteger>  
- IShiftOperators<TSelf,Int32,TSelf>  
- ISignedNumber<BigInteger> 
- ISubtractionOperators<BigInteger,BigInteger,BigInteger>  
- ISubtractionOperators<TSelf,TSelf,TSelf>  
- IUnaryNegationOperators<BigInteger,BigInteger>  
- IUnaryNegationOperators<TSelf,TSelf>  
- IUnaryPlusOperators<BigInteger,BigInteger>  
- IUnaryPlusOperators<TSelf,TSelf>

Розглянемо детальніше кожен із цих інтерфейсів та методи, які мають бути реалізовані в BigInteger.

---

### IComparable<T>, IComparable<BigInteger>  

#### Методи: 
1. **CompareTo(Object)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт.

---

### IEquatable<T>, IEquatable<BigInteger>  

#### Методи:
1. **Equals(T)**: Показує, чи поточний об'єкт дорівнює іншому об'єкту того ж типу.

---

### IFormattable  

Забезпечує функціонал для форматування значення об'єкта у рядкове представлення.

#### Методи:
1. **ToString(String, IFormatProvider)**: Форматує значення поточного екземпляру, використовуючи вказаний формат.

---

### ISpanFormattable  

Забезпечує функціонал для форматування рядкового представлення об'єкта у span.

#### Методи:
1. **ToString(String, IFormatProvider)**: Форматує значення поточного екземпляру, використовуючи вказаний формат. (Успадковано від IFormattable) 
2. **TryFormat(Span<Char>, Int32, ReadOnlySpan<Char>, IFormatProvider)**: Спробує сформатувати значення поточного екземпляру у наданому span символів.

---

### IComparable<TSelf>  

Визначає узагальнений метод порівняння, який значущий тип або клас реалізує для створення типоспецифічного методу порівняння для упорядкування чи сортування його екземплярів.

#### Методи:
1. **CompareTo(T)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт.

---

### IEquatable<TSelf>  

Визначає узагальнений метод, який значущий тип або клас реалізує для створення типоспецифічного методу визначення рівності екземплярів.

#### Методи:
1. **Equals(T)**: Показує, чи поточний об'єкт дорівнює іншому об'єкту того ж типу.

---

### IParsable<BigInteger>, IParsable<TSelf>

Визначає механізм для розбору рядка на значення.

#### Методи:
1. **Parse(String, IFormatProvider)**: Розбирає рядок у значення.
2. **TryParse(String, IFormatProvider, TSelf)**: Намагається розібрати рядок на значення.

---

### ISpanParsable<BigInteger>, ISpanParsable<TSelf>  

Визначає механізм для розбору span символів на значення.

#### Методи:
1. **Parse(ReadOnlySpan<Char>, IFormatProvider)**: Розбирає span символів у значення.
2. **TryParse(ReadOnlySpan<Char>, IFormatProvider, TSelf)**: Намагається розібрати span символів на значення.

---

### IAdditionOperators<TSelf, TOther, TResult>, IAdditionOperators<BigInteger, BigInteger, BigInteger> 

Визначає механізм для обчислення суми двох значень.

#### Методи:
1. **Addition(TSelf, TOther)**: Додає два значення разом для обчислення їх суми.
2. **CheckedAddition(TSelf, TOther)**: Додає два значення разом для обчислення їх суми з перевіркою на переповнення.

---

### IAdditiveIdentity<TSelf, TOther>, IAdditiveIdentity<BigInteger, BigInteger>  

Визначає механізм для отримання адитивної одиниці даного типу.

#### Властивості:
1. **AdditiveIdentity**: Отримує адитивну одиницю поточного типу.

---

### IBinaryInteger<BigInteger>  

Визначає цілочисельний тип, який представлений у двійковому форматі.

#### Методи:
1. **CompareTo(Object)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт. (Успадковано від IComparable) 
2. **CompareTo(T)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт. (Успадковано від IComparable<T>) 
3. **DivRem(TSelf, TSelf)**: Обчислює частку та залишок від ділення двох значень. 
4. **Equals(T)**: Показує, чи поточний об'єкт дорівнює іншому об'єкту того ж типу. (Успадковано від IEquatable<T>) 
5. **GetByteCount()**: Отримує кількість байтів, які будуть записані як частина TryWriteLittleEndian(Span<Byte>, Int32). 
6. **GetShortestBitLength()**: Отримує довжину, в бітах, найкоротшого двійкового представлення двійкового числа поточного значення. 
7. **LeadingZeroCount(TSelf)**: Обчислює кількість ведучих нульових бітів у значенні. 
8. **PopCount(TSelf)**: Обчислює кількість бітів, які встановлені в значенні. 
9. **ReadBigEndian(Byte[], Boolean)**: Зчитує двійкове число з заданого масиву великим порядком та перетворює його в екземпляр поточного типу. 
10. **ReadBigEndian(Byte[], Int32, Boolean)**: Зчитує двійкове число з заданого масиву великим порядком та перетворює його в екземпляр поточного типу. 
11. **ReadBigEndian(ReadOnlySpan<Byte>, Boolean)**: Зчитує двійкове число з заданого span великим порядком та перетворює його в екземпляр поточного типу. 
12. **ReadLittleEndian(Byte[], Boolean)**: Зчитує двійкове число з заданого масиву малим порядком та перетворює його в екземпляр поточного типу. 
13. **ReadLittleEndian(Byte[], Int32, Boolean)**: Зчитує двійкове число з заданого масиву малим порядком та перетворює його в екземпляр поточного типу. 
14. **ReadLittleEndian(ReadOnlySpan<Byte>, Boolean)**: Зчитує двійкове число з заданого span малим порядком та перетворює його в екземпляр поточного типу. 
15. **RotateLeft(TSelf, Int32)**: Повертає значення вліво на вказану кількість бітів. 
16. **RotateRight(TSelf, Int32)**: Повертає значення вправо на вказану кількість бітів. 
17. **ToString(String, IFormatProvider)**: Форматує значення поточного екземпляру використовуючи вказаний формат. (Успадковано від IFormattable) 
18. **TrailingZeroCount(TSelf)**: Обчислює кількість нульових бітів в кінці значення. 
19. **TryFormat(Span<Char>, Int32, ReadOnlySpan<Char>, IFormatProvider)**: Намагається сформатувати значення поточного екземпляру в заданому span символів. (Успадковано від ISpanFormattable) 
20. **TryReadBigEndian(ReadOnlySpan<Byte>, Boolean, TSelf)**: Намагається зчитати двійкове число зі span, використовуючи великий порядок, і перетворити його в екземпляр поточного типу. 
21. **TryReadLittleEndian(ReadOnlySpan<Byte>, Boolean, TSelf)**: Намагається зчитати двійкове число зі span, використовуючи малий порядок, і перетворити його в екземпляр поточного типу. 
22. **TryWriteBigEndian(Span<Byte>, Int32)**: Намагається записати поточне значення, використовуючи великий порядок, у заданий span. 
23. **TryWriteLittleEndian(Span<Byte>, Int32)**: Намагається записати поточне значення, використовуючи малий порядок, у заданий span.
24. **WriteBigEndian(Byte[])**: Записує поточне значення, використовуючи великий порядок, у заданий масив. 
25. **WriteBigEndian(Byte[], Int32)**: Записує поточне значення, використовуючи великий порядок, у заданий масив. 
26. **WriteBigEndian(Span<Byte>)**: Записує поточне значення, використовуючи великий порядок, у заданий span. 
27. **WriteLittleEndian(Byte[])**: Записує поточне значення, використовуючи малий порядок, у заданий масив. 
28. **WriteLittleEndian(Byte[], Int32)**: Записує поточне значення, використовуючи малий порядок, у заданий масив, починаючи з вказаного індексу. 
29. **WriteLittleEndian(Span<Byte>)**: Записує поточне значення, використовуючи малий порядок, у заданий span.

---

### IBinaryNumber<TSelf>, IBinaryNumber<BigInteger>  

Визначає число, яке представлене у двійковому форматі.

#### Властивості:
1. AllBitsSet: Отримує екземпляр двійкового типу, в якому всі біти встановлені.

#### Методи:
1. **CompareTo(Object)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт. (Успадковано від IComparable)
2. **CompareTo(T)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт. (Успадковано від IComparable<T>)
3. **Equals(T)**: Показує, чи поточний об'єкт дорівнює іншому об'єкту того ж типу. (Успадковано від IEquatable<T>) 
4. **IsPow2(TSelf)**: Визначає, чи є значення ступенем двійки.
5. **Log2(TSelf)**: Обчислює логарифм по основі 2 для значення.
6. **ToString(String, IFormatProvider)**: Форматує значення поточного екземпляру використовуючи вказаний формат. (Успадковано від IFormattable)
7. **TryFormat(Span<Char>, Int32, ReadOnlySpan<Char>, IFormatProvider)**: Намагається сформатувати значення поточного екземпляру у заданий span символів. (Успадковано від ISpanFormattable)

---

### IBitwiseOperators<TSelf, TOther, TResult>, IBitwiseOperators<BigInteger, BigInteger, BigInteger>  

Визначає механізм для виконання побітових операцій над двома значеннями.

#### Оператори:
1. **BitwiseAnd(TSelf, TOther)**: Обчислює побітове "і" для двох значень.
2. **BitwiseOr(TSelf, TOther)**: Обчислює побітове "або" для двох значень.
3. **ExclusiveOr(TSelf, TOther)**: Обчислює побітове "виключне або" для двох значень.
4. **OnesComplement(TSelf)**: Обчислює побітове доповнення до одиниці для заданого значення.

---

### IComparisonOperators<TSelf, TOther, Boolean>, IComparisonOperators<BigInteger, BigInteger, Boolean>  

Визначає механізм порівняння двох значень для визначення відносин порядку.

#### Оператори:
1. **GreaterThan(TSelf, TOther)**: Порівнює два значення, щоб визначити, яке з них більше.
2. **GreaterThanOrEqual(TSelf, TOther)**: Порівнює два значення, щоб визначити, яке з них більше або рівне.
3. **LessThan(TSelf, TOther)**: Порівнює два значення, щоб визначити, яке з них менше.
4. **LessThanOrEqual(TSelf, TOther)**: Порівнює два значення, щоб визначити, яке з них менше або рівне.

---

### IDecrementOperators<TSelf>, IDecrementOperators<BigInteger>  

Визначає механізм для зменшення заданого значення.

#### Оператори:
1. **CheckedDecrement(TSelf)**: Зменшує значення з перевіркою на переповнення.
2. **Decrement(TSelf)**: Зменшує значення.

---

### IDivisionOperators<TSelf, TSelf, TResult>, IDivisionOperators<BigInteger, BigInteger, BigInteger>  

Визначає механізм для обчислення частки двох значень.

#### Оператори:
1. **CheckedDivision(TSelf, TOther)**: Ділить два значення разом для обчислення їх частки з перевіркою на переповнення.
2. **Division(TSelf, TOther)**: Ділить одне значення на інше для обчислення їх частки.

---

### IEqualityOperators<TSelf, TOther, Boolean>, IEqualityOperators<TSelf, TOther, TResult>, IEqualityOperators<BigInteger, BigInteger, Boolean>  

Визначає механізм порівняння двох значень для визначення рівності.

#### Оператори:
1. **Equality(TSelf, TOther)**: Порівнює два значення, щоб визначити рівність.
2. **Inequality(TSelf, TOther)**: Порівнює два значення, щоб визначити нерівність.

---

### IIncrementOperators<TSelf>, IIncrementOperators<BigInteger>  

Визначає механізм для збільшення заданого значення.

#### Оператори:
1. **CheckedIncrement(TSelf)**: Збільшує значення з перевіркою на переповнення.
2. **Increment(TSelf)**: Збільшує значення.

---

### IModulusOperators<TSelf, TOther, TResult>, IModulusOperators<BigInteger, BigInteger, BigInteger>

Визначає механізм для обчислення залишку або модуля двох значень.

#### Оператори:
1. **Modulus(TSelf, TOther)**: Ділить два значення разом для обчислення їх залишку або модуля.

---

### IMultiplicativeIdentity<TSelf, TOther>, IMultiplicativeIdentity<BigInteger, BigInteger>

Визначає механізм для отримання множинної одиниці заданого типу.

#### Властивості:
1. **MultiplicativeIdentity**: Отримує множинну одиницю поточного типу.

---

### IMultiplyOperators<TSelf, TOther, TResult>, IMultiplyOperators<BigInteger, BigInteger, BigInteger>  

Визначає механізм для обчислення добутку двох значень.

#### Оператори:
1. **CheckedMultiply(TSelf, TOther)**: Перемножує два значення разом для обчислення їх добутку з перевіркою на переповнення.
2. **Multiply(TSelf, TOther)**: Перемножує два значення разом для обчислення їх добутку.

---

### INumber<TSelf>, INumber<BigInteger>

Визначає тип числа.

#### Методи:
1. **Clamp(TSelf, TSelf, TSelf)**: Обмежує значення включно мінімальним і максимальним значенням.
2. **CompareTo(Object)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт. (Успадковано від IComparable)
3. **CompareTo(T)**: Порівнює поточний екземпляр з іншим об'єктом того ж типу і повертає ціле число, яке вказує, чи передує поточний екземпляр, чи йде за ним, чи знаходиться в тому ж самому положенні в порядку сортування, що інший об'єкт. (Успадковано від IComparable<T>)
4. **CopySign(TSelf, TSelf)**: Копіює знак значення в знак іншого значення.
5. **Equals(T)**: Показує, чи поточний об'єкт дорівнює іншому об'єкту того ж типу. (Успадковано від IEquatable<T>)
6. **Max(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке з них більше.
7. **MaxNumber(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке з них більше і повертає інше значення, якщо вхідне значення є NaN.
8. **Min(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке з них менше.
9. **MinNumber(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке з них менше і повертає інше значення, якщо вхідне значення є NaN.
10. **Sign(TSelf)**: Обчислює знак значення.
11. **ToString(String, IFormatProvider)**: Форматує значення поточного екземпляру використовуючи вказаний формат. (Успадковано від IFormattable)
12. **TryFormat(Span<Char>, Int32, ReadOnlySpan<Char>, IFormatProvider)**: Намагається сформатувати значення поточного екземпляру у заданий span символів. (Успадковано від ISpanFormattable)

---

### INumberBase<TSelf>, INumberBase<BigInteger>

Визначає основу для інших числових типів.

#### Властивості:
1. **One**: Отримує значення 1 для типу.
2. **Radix**: Отримує основу або систему числення для типу.
3. **Zero**: Отримує значення 0 для типу.

#### Методи:
1. **Abs(TSelf)**: Обчислює абсолютне значення.
2. **CreateChecked<TOther>(TOther)**: Створює екземпляр поточного типу зі значенням, викидаючи виняток переповнення для будь-яких значень, які виходять за межі представницького діапазону поточного типу.
3. **CreateSaturating<TOther>(TOther)**: Створює екземпляр поточного типу зі значенням, насичуючи значення, які виходять за межі представницького діапазону поточного типу.
4. **CreateTruncating<TOther>(TOther)**: Створює екземпляр поточного типу зі значенням, обрізаючи значення, які виходять за межі представницького діапазону поточного типу.
5. **Equals(T)**: Вказує, чи поточний об'єкт дорівнює іншому об'єкту того ж типу. (Успадковано від IEquatable<T>)
6. **IsCanonical(TSelf)**: Визначає, чи значення знаходиться в своєму канонічному представленні.
7. **IsComplexNumber(TSelf)**: Визначає, чи значення представляє комплексне число.
8. **IsEvenInteger(TSelf)**: Визначає, чи значення є парним цілим числом.
9. **IsFinite(TSelf)**: Визначає, чи значення є скінченним.
10. **IsImaginaryNumber(TSelf)**: Визначає, чи значення представляє чисто уявне число.
11. **IsInfinity(TSelf)**: Визначає, чи значення є нескінченністю.
12. **IsInteger(TSelf)**: Визначає, чи значення представляє ціле число.
13. **IsNaN(TSelf)**: Визначає, чи значення є NaN (не число).
14. **IsNegative(TSelf)**: Визначає, чи значення представляє від'ємне дійсне число.
15. **IsNegativeInfinity(TSelf)**: Визначає, чи значення є від'ємною нескінченністю.
16. **IsNormal(TSelf)**: Визначає, чи значення є нормальним.
17. **IsOddInteger(TSelf)**: Визначає, чи значення представляє непарне ціле число.
18. **IsPositive(TSelf)**: Визначає, чи значення представляє нуль або додатне дійсне число.
19. **IsPositiveInfinity(TSelf)**: Визначає, чи значення є додатньою нескінченністю.
20. **IsRealNumber(TSelf)**: Визначає, чи значення представляє дійсне число.
21. **IsSubnormal(TSelf)**: Визначає, чи значення є піднормальним.
22. **IsZero(TSelf)**: Визначає, чи значення є нулем.
23. **MaxMagnitude(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке з них більше.
24. **MaxMagnitudeNumber(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке має більший модуль і повертає інше значення, якщо вхідне значення є NaN.
25. **MinMagnitude(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке з них менше.
26. **MinMagnitudeNumber(TSelf, TSelf)**: Порівнює два значення, щоб визначити, яке має менший модуль і повертає інше значення, якщо вхідне значення є NaN.
27. **Parse(ReadOnlySpan<Char>, NumberStyles, IFormatProvider)**: Розбирає діапазон символів у значення.
28. **Parse(String, NumberStyles, IFormatProvider)**: Розбирає рядок у значення.
29. **ToString(String, IFormatProvider)**: Форматує значення поточного екземпляру за вказаним форматом. (Успадковано від IFormattable)
30. **TryConvertFromChecked<TOther>(TOther, TSelf)**: Намагається конвертувати значення в екземпляр поточного типу, викидаючи виняток переповнення для будь-яких значень, які виходять за межі представницького діапазону поточного типу.
31. **TryConvertFromSaturating<TOther>(TOther, TSelf)**: Намагається конвертувати значення в екземпляр поточного типу, насичуючи значення, які виходять за межі представницького діапазону поточного типу.
32. **TryConvertFromTruncating<TOther>(TOther, TSelf)**: Намагається конвертувати значення в екземпляр поточного типу, обрізаючи значення, які виходять за межі представницького діапазону поточного типу.
33. **TryConvertToChecked<TOther>(TSelf, TOther)**: Намагається конвертувати екземпляр поточного типу в інший тип, викидаючи виняток переповнення для будь-яких значень, які виходять за межі представницького діапазону поточного типу.
34. **TryConvertToSaturating<TOther>(TSelf, TOther)**: Намагається конвертувати екземпляр поточного типу в інший тип, насичуючи значення, які виходять за межі представницького діапазону поточного типу.
35. **TryConvertToTruncating<TOther>(TSelf, TOther)**: Намагається конвертувати екземпляр поточного типу в інший тип, обрізаючи значення, які виходять за межі представницького діапазону поточного типу.
36. **TryFormat(Span<Char>, Int32, ReadOnlySpan<Char>, IFormatProvider)**: Намагається сформатувати значення поточного екземпляру в наданому діапазоні символів. (Успадковано від ISpanFormattable)
37. **TryParse(ReadOnlySpan<Char>, NumberStyles, IFormatProvider, TSelf)**: Намагається розібрати діапазон символів у значення.
38. **TryParse(String, NumberStyles, IFormatProvider, TSelf)**: Намагається розібрати рядок у значення.

---

### IShiftOperators<TSelf,Int32,TSelf>, IShiftOperators<BigInteger,Int32,BigInteger>

Визначає механізм для зсуву значення на інше значення.

#### Оператори:
1. **LeftShift(TSelf, TOther)**: Здвигає значення вліво на задану кількість позицій.
2. **RightShift(TSelf, TOther)**: Здвигає значення вправо на задану кількість позицій.
3. **UnsignedRightShift(TSelf, TOther)**: Здвигає значення вправо на задану кількість позицій без знака.

---

### ISignedNumber<BigInteger>

Визначає тип числа, який може представляти як позитивні, так і негативні значення.

#### Властивості:
1. **NegativeOne**: Отримує значення -1 для цього типу.

#### Методи:
1. **Equals(T)**: Показує, чи поточний об'єкт рівний іншому об'єкту того ж типу. (Успадковано від IEquatable<T>)
2. **ToString(String, IFormatProvider)**: Форматує значення поточного екземпляру за вказаним форматом. (Успадковано від IFormattable)
3. **TryFormat(Span<Char>, Int32, ReadOnlySpan<Char>, IFormatProvider)**: Намагається сформатувати значення поточного екземпляру в наданому діапазоні символів. (Успадковано від ISpanFormattable)

---

### ISubtractionOperators<TSelf,TSelf,TSelf>, ISubtractionOperators<BigInteger,BigInteger,BigInteger>

Визначає механізм для обчислення різниці двох значень.

#### Оператори:
1. **CheckedSubtraction(TSelf, TOther)**: Віднімає два значення для обчислення їх різниці.
2. **Subtraction(TSelf, TOther)**: Віднімає два значення для обчислення їх різниці.

---

### IUnaryNegationOperators<TSelf,TSelf>, IUnaryNegationOperators<BigInteger,BigInteger>

Визначає механізм для обчислення унарної від'ємності значення.

#### Оператори:
1. **CheckedUnaryNegation(TSelf)**: Обчислює перевірену унарну від'ємність значення.
2. **UnaryNegation(TSelf)**: Обчислює унарну від'ємність значення.

---

### IUnaryPlusOperators<TSelf,TSelf>, IUnaryPlusOperators<BigInteger,BigInteger>

Визначає механізм для обчислення унарного плюсу значення.

#### Оператори:
1. **UnaryPlus(TSelf)**: Обчислює унарний плюс значення.