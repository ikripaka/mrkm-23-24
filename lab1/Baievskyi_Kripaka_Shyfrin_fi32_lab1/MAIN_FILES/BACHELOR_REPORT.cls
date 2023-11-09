% ********** Клас для написання звіту з виконаної роботи **********
% Автор: Сергій Яковлєв
% Зауваження: клас складається з копіпасти та костилів трошки більше аніж 
%             повністю; ПОРОБЛЕNО і все таке, але повинно працювати
%            (власне, це майже повністю клас бакалаврської дипломної 
%             роботи)

% 1. Ідентифікуємо клас
\ProvidesClass{bachelor_report}[2021/05/17 v0.2b Звіт з виконаної роботи]
\NeedsTeXFormat{LaTeX2e} % потрібний формат


% 2. Завантажуємо "батьківський клас"
\LoadClass[a4paper,ukrainian,oneside]{extreport}


% 3. Завантажуємо пакети

% мовні пакети
\RequirePackage{cmap}                                   % для кодування шрифтів у pdf
\RequirePackage[T2A]{fontenc}                           % пакет для підтримки кирилічних шрифтів
\RequirePackage[utf8]{inputenc}                         % пакет для підтримки Windows'івського кодування cp1251
\RequirePackage[russian,ukrainian]{babel}               % пакет для підтримки української та російської мов
\RequirePackage{mathtext}                               % потрібні символи в режимі математичних формул
\RequirePackage{amssymb,amsfonts,amsmath,amsthm}        % потрібні символи в режимі математичних формул
\RequirePackage{textcomp}                               % потрібні символи в режимі математичних формул
\RequirePackage{upgreek}                                % пакет для прямих грецьких літер
%\RequirePackage{icomma}                                 % пакет для нормальних пробілів після ком у десяткових дробах
% --! Пакет icomma кофліктує з пакетами subfig та subcaption
% --! У разі помилок закоментуйте пакет icomma


% пакети для форматування відступів та полів
\RequirePackage[includehead, headsep=0pt, footskip=0pt, top=2cm, bottom=2cm, left=2.5cm, right=1.5cm]{geometry} 
                                                        % задаємо поля сторінки
\RequirePackage{indentfirst}                            % відступ для першого рядка абзацу
\RequirePackage{hanging}                                % відступ для всіх інших рядків абзацу
\RequirePackage[shortlabels]{enumitem}                  % пакет для форматування списків
\RequirePackage{setspace}                               % пакет для того, щоб можна було задати міжрядковий інтервал
\RequirePackage{fancyhdr}                               % пакет потрібний для виставлення позиції номеру сторінки
\RequirePackage{etoolbox}                               % пакет для прибирання висячого відступу в бібліографії та прописування умовних відстаней між заголовками

% графічні пакети та таблиці
\RequirePackage{graphicx}               % основний пакет для вставки графіки
%\RequirePackage{subfig}                 % пакет для вставки підрисунків
\RequirePackage{subcaption}             % пакет для вставки підрисунків (subfig та subcaption конфліктують між собою)
\RequirePackage{flafter}                % пакет гарантує, що рисунок чи таблиця ніколи не з'являться раніше посилання на самих себе
\RequirePackage{caption}                % пакет для персоналізації заголовків рисунків та таблиць
\RequirePackage{xtab}                   % пакет для довгих таблиць
\RequirePackage{longtable}              % пакет для довгих таблиць
\RequirePackage{multirow}               % пакет для об'єднання рядків таблиці
\RequirePackage{array}                  % пакет для вирівнювання всередині таблиці

% стилі заголовків та змісту
\RequirePackage[rm,small,raggedright]{titlesec}         % пакет для ручного керування стилем заголовків
\RequirePackage[titles,subfigure]{tocloft}              % пакет для ручного керування стилем змісту
%\RequirePackage[titles]{tocloft}              % пакет для ручного керування стилем змісту

\RequirePackage{url}                    % пакет для коректного представлення інтернет-посилань у "Переліку посилань"
\RequirePackage[hidelinks]{hyperref}    % пакет для гіперпосилань у змісті

\RequirePackage{listings}               % пакет для відображення форматування вихідних кодів
\RequirePackage{verbatim}               % пакет для кодів програм
\RequirePackage{xkeyval}                % пакет для параметричних команд


% ************************************************************* %
%  4. Основна частина
% ************************************************************* %

% початкові налаштування

\frenchspacing
\doublespacing          % інтервал "1,5" між рядками -- \linestretch ??

\newcommand{\setfontsize}[1]{\fontsize{#1pt}{#1pt}\selectfont}  % команда для задавання розміру шрифта

\urlstyle{same}         % текст гіперпосилань буде того ж шрифту, що й основний текст

% стиль сторінки для позиції номеру сторінки
\fancypagestyle{plain}{
        \fancyhf{}
        \rhead{\thepage}
                \renewcommand{\headrulewidth}{0pt}
                \renewcommand{\footrulewidth}{0pt}
}


% команди для заборони використання перенесень
\tolerance=1
\emergencystretch=\maxdimen
\hyphenpenalty=10000
\hbadness=10000

\setlength{\parindent}{1.25cm} % задаємо відступ першого рядка абзацу (5 знаків, або 1,25 см)

% пропуск рядка до та після формули
\setlength{\belowdisplayskip}{21pt}
\setlength{\abovedisplayskip}{21pt}
\setlength{\belowdisplayshortskip}{21pt}
\setlength{\abovedisplayshortskip}{21pt}


% ************************************************************* %
%  Оформлення списків, таблиць, рисунків
% ************************************************************* %

% задамо оновлені списки, з правильними відступами
% нумерований першого рівня
\AddEnumerateCounter{\asbuk}{\@asbuk}{\cyrm}
\setlist[itemize,1]{label=--, partopsep=0pt, parsep=0pt, topsep=0pt, itemsep=0pt, itemindent=1.7cm, leftmargin=0pt}
\setlist[itemize,2]{label=--, partopsep=0pt, parsep=0pt, topsep=0pt, itemsep=0pt, itemindent=2.25cm, leftmargin=0pt}
\setlist[enumerate,1]{label=\arabic*), partopsep=0pt, parsep=0pt, topsep=0pt, itemsep=0pt, itemindent=1.9cm, leftmargin=0pt}
\setlist[enumerate,2]{label=\asbuk*), partopsep=0pt, parsep=0pt, topsep=0pt, itemsep=0pt, itemindent=2.6cm, leftmargin=0pt}

% заголовки таблиць, рисунків
\DeclareCaptionLabelFormat{tablecap}{\hspace*{1cm}Таблиця #2} % формат за промовчанням "Табл." не влаштовує
\DeclareCaptionLabelFormat{figurecap}{Рисунок #2} % формат за промовчанням "Рис." не влаштовує

%\renewcommand{\thesubfigure}{\asbuk{subfigure}}

\captionsetup[table]{margin=0pt, singlelinecheck=false, labelfont=bf, textfont=normalfont, justification=justified, labelsep=endash, labelformat=tablecap, font = {Large, stretch = 1.5}} % формат для таблиць
\captionsetup[figure]{justification=centering, labelsep=endash, labelfont=bf, textfont=normalfont, labelformat=figurecap, font = {Large, stretch = 1.5}} % формат для рисунків
%\captionsetup[lstlisting]{margin=0pt, format=listing, singlelinecheck=false, font = {stretch = 1.5}} % формат для лістингів

\setlength{\textfloatsep}{21pt}
\setlength{\intextsep}{21pt}

% нове оточення для довгих таблиць
\newenvironment{ltable}[4]{
        \begingroup\centering
        \setlength{\LTpre}{-14pt}
        \setlength{\LTpost}{14pt}
        \setlength{\LTcapwidth}{0pt}
        \begin{longtable}{#1}
                \kill
                \caption{#2}\label{#3}\\
                #4
                \endfirsthead
                \kill
                \caption*{Продовження таблиці \thetable}\\
                        #4
                        \endhead
                        \hline
                        \endfoot
        }
        {%
                \end{longtable}
                \endgroup       
        }


% ************************************************************* %
%  Визначення математичних оточень та роботи з формулами
% ************************************************************* %

\theoremstyle{plain}
\newtheorem{theorem}{\indent Теорема}[chapter]
\newtheorem{claim}{\indent Твердження}[chapter]
\newtheorem{lemma}{\indent Лема}[chapter]
\newtheorem{corollary}{\indent Наслідок}[chapter]

\theoremstyle{definition}
\newtheorem{definition}{\indent Означення}[chapter]
\newtheorem{algorithm}{\indent Алгоритм}[chapter]
\newtheorem{problem}{\indent Задача}[chapter]
\newtheorem{example}{\indent Приклад}[chapter]
\theoremstyle{remark}
\newtheorem*{remark}{\indent\textbf{Зауваження}}
\renewenvironment{proof}{{\indent\bfseries Доведення.~}}{\qed}

% змінюємо формат нумерації формул
\renewcommand{\theequation}{\arabic{chapter}.\arabic{equation}}

% команди для заборони розриву формул у небажаному місці
\binoppenalty=10000
\relpenalty=10000

% виправляємо помилки з апострофами в формулах
\makeatletter
\def\latex@@rquote{^\bgroup\prim@s} 
\let\hang@@h@ngrquote\h@ngrquote
\renewcommand{\h@ngrquote}{
        \relax\ifmmode
                \expandafter\latex@@rquote
        \else
                \expandafter\hang@@h@ngrquote
        \fi}
\makeatother


% ************************************************************* %
%  Ненумеровані розділи
% ************************************************************* %

% команда для створення ненумерованого розділу, який тим не менше показується в змісті
\newcommand{\uchapter}[1]{\chapter*{#1}\addcontentsline{toc}{chapter}{#1}} 

\newcommand{\intro}{\uchapter{Вступ}}                   % для створення вступу використаємо команду \uchapter
\newcommand{\conclusions}{\uchapter{Висновки}}          % для створення висновків використаємо команду \uchapter
\newcommand{\shortings}{\uchapter{Перелік умовних позначень, скорочень і термінів}}
                                                        % для створення переліку умовних позначень використаємо команду \shortings

\newcommand{\chapconclude}[1]{\section*{Висновки до розділу #1} \addcontentsline{toc}{section}{Висновки до розділу #1}}

% нові команди для анотацій
% трьома мовами
\newcommand{\abstractUkr}{\chapter*{РЕФЕРАТ}\thispagestyle{plain}\renewcommand{\arraystretch}{1.5}\setfontsize{14}}
\newcommand{\abstractRus}{\chapter*{РЕФЕРАТ}\thispagestyle{plain}\renewcommand{\arraystretch}{1.5}\setfontsize{14}}
\newcommand{\abstractEng}{\chapter*{ABSTRACT}\thispagestyle{plain}\renewcommand{\arraystretch}{1.5}\setfontsize{14}}


% ************************************************************* %
%  Додатки
% ************************************************************* %

% хитра команда для створення додатків
\newcounter{appendnum}
\newcommand{\append}[1]{
        \setcounter{chapter}{0}
        \setcounter{figure}{0}
        \setcounter{table}{0}
        \setcounter{section}{0}
        \setcounter{subsection}{0}
        \titleformat{\chapter}
                {\centering\Large\bfseries\MakeUppercase}
                {\hspace{1cm}\thechapter}
                {0.5em}
                {}
        \chapter*{ДОДАТОК \Asbuk{appendnum}~~#1}
        \addcontentsline{toc}{chapter}{Додаток \Asbuk{appendnum}\ #1}
        \addvspace{-28pt}
        \preto\section{%
                \ifnum\value{section}=0\addvspace{0pt}\fi%
        }
        \renewcommand{\thefigure}{\Asbuk{appendnum}.\arabic{figure}}
        \renewcommand{\thetable}{\Asbuk{appendnum}.\arabic{table}}
        \renewcommand{\thesection}{\Asbuk{appendnum}.\arabic{section}}
        \renewcommand{\thesubsection}{\Asbuk{appendnum}.\arabic{section}.\arabic{subsection}}
}
% прибираємо недопустимі букви з позначення додатків
\pretocmd\append{
        \stepcounter{appendnum}
        \ifnum\Asbuk{appendnum}=Ґ \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=Є \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=З \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=І \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=Ї \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=Й \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=О \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=Ч \stepcounter{appendnum}\fi
        \ifnum\Asbuk{appendnum}=Ь \stepcounter{appendnum}\fi
}{}{}


% ************************************************************* %
%  Заголовки розділів
% ************************************************************* %

% спочатку --- для розділу
\titleformat{\chapter} % указуємо, що модифікуємо саме розділ
                {\centering\Large\MakeUppercase}        % указуємо формат назви ("усі великі")
                {\bfseries\thechapter}                  % указуємо формат власне номера: це буде просто число, без крапки
                {0.5em}                                 % відстань між номером і назвою
                {\bfseries}                             % текст, що передує назві
        
% тепер --- для підрозділу
\titleformat{\section}[block] % указуємо, що модифікуємо саме підрозділ
                {\Large}                        % указуємо формат назви (жирний)
                {\hspace*{1cm}\bfseries\thesection}          % указуємо формат власне номера: це буде просто число, без крапки
                {0.5em}                         % відстань між номером і назвою
                {\bfseries}                     % текст, що передує назві

% тепер --- для підрозділу ще нижчого рівня
\titleformat{\subsection}[block] % указуємо, що модифікуємо саме підрозділ нижнього рівня
                {\Large}                        % указуємо формат назви (жирний)
                {\bfseries\thesubsection}       % указуємо формат власне номера: це буде просто число, без крапки
                {0.5em}                         % відстань між номером і назвою
                {\bfseries}                     % текст, що передує назві
                        
% тепер задамо відступи для заголовку розділу
\titlespacing{\chapter} % указуємо, що модифікуємо саме розділ
                {0pt}           % відступ зліва
                {-28pt}         % відступ зверху (немає)
                {48pt}          % відступ знизу (2 рядки)

% тепер задамо відступи для заголовку підрозділу                        
\titlespacing{\section} % указуємо, що модифікуємо саме підрозділ
%                {\parindent}    % відступ зліва
%                {0pt}    % відступ зліва
                {0.5em}    % відступ зліва
                {24pt}          % відступ зверху (2 рядки)
                {24pt}          % відступ знизу (1 рядок)


%% тепер задамо відступи для заголовку підрозділу без нумерації                  
\titlespacing{name=\section,numberless} 
                {\parindent}    % відступ зліва
                {24pt}          % відступ зверху (2 рядки)
                {24pt}          % відступ знизу (1 рядок)

                        
% тепер задамо відступи для заголовку пункта
\titlespacing{\subsection} % указуємо, що модифікуємо саме пункт
                {\parindent}    % відступ зліва
                {24pt}          % відступ зверху (2 рядки)
                {24pt}          % відступ знизу (1 рядок)


% тепер зробимо так, щоб між двома заголовками була звичайна відстань
\preto\section{%
        \ifnum\value{section}=0\addvspace{-24pt}\fi%
}

\preto\subsection{%
        \ifnum\value{subsection}=0\addvspace{-24pt}\fi%
}


% ************************************************************* %
%  Зміст
% ************************************************************* %

% видозмінюємо зміст
\makeatletter
\renewcommand{\tableofcontents}{
        \clearpage
        \thispagestyle{empty}
        \begin{center}
                \bfseries\MakeUppercase{Зміст}
        \end{center}

        \addvspace{42pt}
        \@starttoc{toc}
}
\makeatother

% задаємо відстань між точками
\renewcommand{\cftchapdotsep}{1}
\renewcommand{\cftsecdotsep}{1}
\renewcommand{\cftsubsecdotsep}{1}

% задаємо розмір поля, що виділяється під номер сторінки
\cftsetpnumwidth{1em}

% задамо відстані між рядками в змісті
\setlength\cftbeforechapskip{0pt}

% займемося розділами
\renewcommand{\cftchapaftersnum}{}              % що буде писатися після номеру розділу (нічого)
\renewcommand{\cftchapindent}{0cm}              % відступ номера розділу від лівого краю
\renewcommand{\cftchapnumwidth}{1em}            % розмір поля, що виділяється під номер розділу
\renewcommand{\cftchappagefont}{\mdseries}      % стиль номеру сторінки (звичайний, 14pt)
\renewcommand{\cftchapfont}{\mdseries}          % стиль заголовку (звичайний)
\renewcommand{\cftchapleader}{\cftdotfill{\cftchapdotsep}} % чим заповнювати проміжок від заголовку до номеру сторінки (точками)

% тепер займемося підрозділами
\renewcommand{\cftsecaftersnum}{}               % що буде писатися після номеру підрозділу (нічого)
\renewcommand{\cftsecindent}{1em}               % відступ номера підрозділу від лівого краю
\renewcommand{\cftsecnumwidth}{2em}             % розмір поля, що виділяється під номер підрозділу
\renewcommand{\cftsecpagefont}{\mdseries}       % стиль номеру сторінки (звичайний, 14pt)
\renewcommand{\cftsecfont}{\mdseries}           % стиль заголовку (звичайний)
\renewcommand{\cftsecleader}{\cftdotfill{\cftsecdotsep}} % чим заповнювати проміжок від заголовку до номеру сторінки (точками)

% тепер займемося пунктами
\renewcommand{\cftsubsecaftersnum}{}            % що буде писатися після номеру пункта (нічого)
\renewcommand{\cftsubsecindent}{3em}            % відступ номера пункта від лівого краю
\renewcommand{\cftsubsecnumwidth}{2.5em}        % розмір поля, що виділяється під номер пункта
\renewcommand{\cftsubsecpagefont}{\mdseries}    % стиль номеру сторінки (звичайний, 14pt)
\renewcommand{\cftsubsecfont}{\mdseries}        % стиль заголовку (звичайний)
\renewcommand{\cftsubsecleader}{\cftdotfill{\cftsubsecdotsep}} % чим заповнювати проміжок від заголовку до номеру сторінки (точками)


% ************************************************************* %
%  Бібліографія
% ************************************************************* %

% для створення переліку використаної літератури \bibliographylist
\newcommand{\bibliographylist}{\uchapter{Література}}


% наведемо порядок із відступами в бібліографії
\patchcmd{\thebibliography}
        {\advance\leftmargin\labelsep}
        {\leftmargin=0pt\itemindent=1.9cm\advance\itemindent\labelsep}
        {}
        {}

% змінюємо формат номеру для переліку посилань (число + крапка)
\makeatletter
\renewcommand{\@biblabel}[1]{#1.}
\makeatother

% оновимо стандартне оточення, оскільки потрібно додати перелік посилань у зміст
\let\tempbib\thebibliography
\let\tempbibend\endthebibliography
\renewenvironment{thebibliography}{
        \renewcommand{\bibname}{ПЕРЕЛІК ПОСИЛАНЬ}% змінюємо заголовок
        \tempbib{99.}           % максимально можливий номер джерела --- 99
        % далі наведемо порядок із відступами
        \setlength\partopsep{0pt}
        \setlength\parskip{0pt}
        \setlength\parsep{0pt}
        \setlength\topsep{0pt}
        \setlength\itemsep{0pt}
        \addcontentsline{toc}{chapter}{Перелік посилань}        % додаємо в зміст
}
{
        \tempbibend
}


