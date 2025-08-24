FIPS  203 

Federal  Information  Processing  Standards  Publication 

Module-Lattice-Based 
Key-Encapsulation  Mechanism  Standard 

Category:  Computer  Security 

Subcategory:  Cryptography 

Information Technology Laboratory 
National Institute of Standards and Technology 
Gaithersburg, MD 20899-8900 

This publication is available free of charge from: 
https://doi.org/10.6028/NIST.FIPS.203 

Published August 13, 2024 

U.S.  Department  of  Commerce 
Gina M. Raimondo, Secretary 

National  Institute  of  Standards  and  Technology 
Laurie E. Locascio, NIST Director and Under Secretary of Commerce for Standards and Technology 

Foreword 

The Federal Information Processing Standards (FIPS) Publication Series of the National Institute of Stan-
dards and Technology is the official series of publications relating to standards and guidelines developed 
under 15 U.S.C. 278g-3, and issued by the Secretary of Commerce under 40 U.S.C. 11331. 

Comments concerning this Federal Information Processing Standard publication are welcomed and should 
be submitted using the contact information in the “Inquiries and Comments” clause of the announcement 
section. 

Kevin M. Stine, Director 
Information Technology Laboratory 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Abstract 
A key-encapsulation mechanism (KEM) is a set of algorithms that, under certain conditions, can be 
used by two parties to establish a shared secret key over a public channel. A shared secret key that 
is securely established using a KEM can then be used with symmetric-key cryptographic algorithms 
to perform basic tasks in secure communications, such as encryption and authentication. This 
standard specifies a key-encapsulation mechanism called ML-KEM. The security of ML-KEM is 
related to the computational difficulty of the Module Learning with Errors problem. At present, 
ML-KEM is believed to be secure, even against adversaries who possess a quantum computer. 
This standard specifies three parameter sets for ML-KEM. In order of increasing security strength 
and decreasing performance, these are ML-KEM-512, ML-KEM-768, and ML-KEM-1024. 

Keywords: computer security; cryptography; encryption; Federal Information Processing Stan-
dards; key-encapsulation mechanism; lattice-based cryptography; post-quantum; public-key 
cryptography. 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Federal  Information  Processing  Standards  Publication  203 

Published:  August  13,  2024 
Effective:  August  13,  2024 

Announcing  the 
Module-Lattice-Based  Key-Encapsulation
Mechanism  Standard 

Federal Information Processing Standards (FIPS) publications are developed by the National 
Institute of Standards and Technology (NIST) under 15 U.S.C. 278g-3 and issued by the Secretary 
of Commerce under 40 U.S.C. 11331. 

1.  Name of Standard.  Module-Lattice-Based Key-Encapsulation Mechanism Standard (FIPS 

203). 

2.  Category of Standard. Computer Security. Subcategory. Cryptography. 

3.  Explanation. A cryptographic key (or simply “key”) is represented in a computer as a string of 
bits. A shared secret key is a cryptographic key that is computed jointly by two parties (e.g., 
Alice and Bob) using a set of algorithms. Under certain conditions, these algorithms ensure 
that both parties will produce the same key and that this key is secret from adversaries. Such 
a shared secret key can then be used with symmetric-key cryptographic algorithms (specified 
in other NIST standards) to perform tasks such as encryption and authentication of digital 
information. 

This standard specifies a set of algorithms for establishing a shared secret key. While there 
are many methods for establishing a shared secret key, the particular method described in 
this standard is a key-encapsulation mechanism (KEM). 

In a KEM, the computation of the shared secret key begins with Alice generating a decapsu-
lation key and an encapsulation key. Alice keeps the decapsulation key private and makes 
the encapsulation key available to Bob. Bob then uses Alice’s encapsulation key to generate 
one copy of a shared secret key along with an associated ciphertext.  Bob then sends the 
ciphertext to Alice.  Finally, Alice uses the ciphertext from Bob along with Alice’s private 
decapsulation key to compute another copy of the shared secret key. 

The security of the particular KEM specified in this standard is related to the computational 
difficulty of solving certain systems of noisy linear equations, specifically the Module Learn-
ing With Errors (MLWE) problem.  At present, it is believed that this particular method of 
establishing a shared secret key is secure, even against adversaries who possess a quantum 
computer. In the future, additional KEMs may be specified and approved in FIPS publications 
or in NIST Special Publications. 

4.  Approving Authority. Secretary of Commerce. 

5.  Maintenance Agency. Department of Commerce, National Institute of Standards and Tech-

nology, Information Technology Laboratory (ITL). 

i 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

6.  Applicability. Federal Information Processing Standards apply to information systems used or 
operated by federal agencies or by a contractor of an agency or other organization on behalf 
of an agency. They do not apply to national security systems as defined in 44 U.S.C. 3552. 

This standard, or other FIPS or NIST Special Publications that specify alternative mechanisms, 
shall be used wherever the establishment of a shared secret key (or shared secret from which 
keying material can be generated) is required for federal applications, including the use of 
such a key with symmetric-key cryptographic algorithms, in accordance with applicable Office 
of Management and Budget and agency policies. 

The adoption and use of this standard are available to private and commercial organizations. 

7.  Implementations. A key-encapsulation mechanism may be implemented in software, firmware, 
hardware, or any combination thereof. For every computational procedure that is specified 
in this standard, a conforming implementation may replace the given set of steps with any 
mathematically equivalent set of steps. In other words, different procedures that produce 
the correct output for every input are permitted. 

NIST will develop a validation program to test implementations for conformance to the 
algorithms in this standard.  Information about validation programs is available at https: 
//csrc.nist.gov/projects/cmvp. Example values will be available at https://csrc.nist.gov/proj 
ects/cryptographic-standards-and-guidelines/example-values. 

8.  Other  Approved  Security  Functions.  Implementations  that  comply  with  this  standard 
shall employ cryptographic algorithms that have been approved for protecting Federal 
Government-sensitive information.  Approved cryptographic algorithms and techniques 
include those that are either: 

(a)  Specified in a Federal Information Processing Standards (FIPS) publication, 

(b)  Adopted in a FIPS or NIST recommendation, or 

(c)  Specified in the list of approved security functions in SP 800-140C. 

9.  Export Control. Certain cryptographic devices and technical data regarding them are subject 
to federal export controls. Exports of cryptographic modules that implement this standard 
and technical data regarding them must comply with all federal laws and regulations and 
be licensed by the Bureau of Industry and Security of the U.S. Department of Commerce. 
Information about export regulations is available at https://www.bis.doc.gov. 

10. Patents. NIST has entered into two patent license agreements to facilitate the adoption of 
NIST’s announced selection of the PQC key-encapsulation mechanism CRYSTALS-KYBER. NIST 
and the licensing parties share a desire, in the public interest, the licensed patents be freely 
available to be practiced by any implementer of the ML-KEM algorithm as published by NIST. 
ML-KEM is the name given to the algorithm in this standard derived from CRYSTALS-KYBER. 
For a summary and extracts from the license, please see https://csrc.nist.gov/csrc/media/P 
rojects/post-quantum-cryptography/documents/selected-algos-2022/nist-pqc-license-sum 
mary-and-excerpts.pdf. Implementation of the algorithm specified in the standard may be 
covered by U.S. and foreign patents of which NIST is not aware. 

ii 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

11. Implementation Schedule. This standard becomes effective immediately upon final publica-

tion. 

12. Specifications. Federal Information Processing Standards (FIPS) 203, Module-Lattice-Based 

Key-Encapsulation Mechanism Standard (affixed). 

13. Qualifications.  In applications, the security guarantees of a KEM only hold under certain 
conditions (see SP 800-227 [1]). One such condition is the secrecy of several values, including 
the randomness used by the two parties, the decapsulation key, and the shared secret key 
itself. Users shall, therefore, guard against the disclosure of these values. 

While it is the intent of this standard to specify general requirements for implementing 
ML-KEM algorithms, conformance to this standard does not ensure that a particular imple-
mentation is secure. It is the responsibility of the implementer to ensure that any module 
that implements a key establishment capability is designed and built in a secure manner. 

Similarly, the use of a product containing an implementation that conforms to this standard 
does not guarantee the security of the overall system in which the product is used. The re-
sponsible authority in each agency or department shall ensure that an overall implementation 
provides an acceptable level of security. 

NIST will continue to follow developments in the analysis of the ML-KEM algorithm. As with 
its other cryptographic algorithm standards, NIST will formally reevaluate this standard every 
five years. 

Both this standard and possible threats that reduce the security provided through the use of 
this standard will undergo review by NIST as appropriate, taking into account newly available 
analysis and technology. In addition, the awareness of any breakthrough in technology or 
any mathematical weakness of the algorithm will cause NIST to reevaluate this standard and 
provide necessary revisions. 

14. Waiver Procedure. The Federal Information Security Management Act (FISMA) does not allow 
for waivers to Federal Information Processing Standards (FIPS) that are made mandatory by 
the Secretary of Commerce. 

15. Where to Obtain Copies of the Standard. This publication is available by accessing https: 
//csrc.nist.gov/publications. Other computer security publications are available at the same 
website. 

16. How to Cite This Publication. NIST has assigned NIST FIPS 203 as the publication identifier 
for this FIPS, per the NIST Technical Series Publication Identifier Syntax. NIST recommends 
that it be cited as follows: 

National Institute of Standards and Technology (2024) Module-Lattice-Based Key-
Encapsulation Mechanism Standard.  (Department of Commerce, Washington, 
D.C.), Federal Information Processing Standards Publication (FIPS) NIST FIPS 203. 
https://doi.org/10.6028/NIST.FIPS.203 

17. Inquiries and Comments.  Inquiries and comments about this FIPS may be submitted to 

fips-203-comments@nist.gov. 

iii 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Federal  Information  Processing  Standards  Publication  203 

Specification  for  the 
Module-Lattice-Based  Key-Encapsulation 
Mechanism  Standard 

Table of Contents 

1 

Introduction 

1.1  Purpose and Scope 

1.2  Context 

2  Terms, Acronyms, and Notation 

2.1  Terms and Definitions 

2.2  Acronyms 

2.3  Mathematical Symbols 

2.4 

Interpreting the Pseudocode 
2.4.1  Data Types 
2.4.2 
Loop Syntax 
2.4.3  Arithmetic With Arrays of Integers 
2.4.4  Representations of Algebraic Objects 
2.4.5  Arithmetic With Polynomials and NTT Representations 
2.4.6  Matrices and Vectors 
2.4.7  Arithmetic With Matrices and Vectors 
2.4.8  Applying Algorithms to Arrays, Examples 

3  Overview of the ML-KEM Scheme 

3.1  Key-Encapsulation Mechanisms 

3.2  The ML-KEM Scheme 

3.3  Requirements for ML-KEM Implementations 

4  Auxiliary Algorithms 

4.1  Cryptographic Functions 

4.2  General Algorithms 

4.2.1  Conversion and Compression Algorithms 
4.2.2 

Sampling Algorithms 

4.3  The Number-Theoretic Transform 

iv 

1 

1 

1 

2 

2 

4 

5 

6 
7 
7 
7 
8 
9 
9 
10 
11 

12 

12 

13 

15 

18 

18 

20 
20 
22 
24 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

4.3.1  Multiplication in the NTT Domain 

5  The K-PKE Component Scheme 

5.1  K-PKE Key Generation 

5.2  K-PKE Encryption 

5.3  K-PKE Decryption 

6  Main Internal Algorithms 

6.1 

Internal Key Generation 

6.2 

Internal Encapsulation 

6.3 

Internal Decapsulation 

7  The ML-KEM Key-Encapsulation Mechanism 

7.1  ML-KEM Key Generation 

7.2  ML-KEM Encapsulation 

7.3  ML-KEM Decapsulation 

8  Parameter Sets 

References 

Appendix A — Precomputed Values for the NTT 

Appendix B — SampleNTT Loop Bounds 

Appendix C — Differences From the CRYSTALS-KYBER Submission 

C.1  Differences Between CRYSTALS-KYBER and FIPS 203 Initial Public Draft 

C.2  Changes From FIPS 203 Initial Public Draft 

27 

28 

28 

29 

31 

32 

32 

32 

33 

35 

35 

36 

37 

39 

41 

44 

46 

47 

47 

47 

v 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

List of Tables 

Table 1 
Table 2 
Table 3 
Table 4  While-loop limits and probabilities of occurrence for SampleNTT 

Decapsulation failure rates for ML-KEM 
Approved parameter sets for ML-KEM 
Sizes (in bytes) of keys and ciphertexts of ML-KEM 

List of Figures 

Figure 1 

A simple view of key establishment using a KEM 

List of Algorithms 

. . . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 1 
ForExample() 
SHAKE128example(str1, … , str𝑚, 𝑏1, … , 𝑏ℓ) 
. . . . . . . . . . . . . 
Algorithm 2 
. . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 3 
BitsToBytes(𝑏) 
BytesToBits(𝐵)  . . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 4 
ByteEncode𝑑(𝐹 )  . . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 5 
ByteDecode𝑑(𝐵) . . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 6 
SampleNTT(𝐵)  . . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 7 
SamplePolyCBD𝜂(𝐵)  . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 8 
. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 9 
NTT(𝑓) 
Algorithm 10  NTT−1(𝑓)̂ 
. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 11  MultiplyNTTs(𝑓,̂ 𝑔)̂ 
. . . . . . . . . . . . . . . . . . . . . . . . . . 
. . . . . . . . . . . . . . . . . . 
Algorithm 12  BaseCaseMultiply(𝑎0, 𝑎1, 𝑏0, 𝑏1, 𝛾) 
Algorithm 13  K-PKE.KeyGen(𝑑) 
. . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 14  K-PKE.Encrypt(ekPKE, 𝑚, 𝑟) . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 15  K-PKE.Decrypt(dkPKE, 𝑐)  . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 16  ML-KEM.KeyGen_internal(𝑑, 𝑧)  . . . . . . . . . . . . . . . . . . . . 
Algorithm 17  ML-KEM.Encaps_internal(ek, 𝑚) 
. . . . . . . . . . . . . . . . . . . 
Algorithm 18  ML-KEM.Decaps_internal(dk, 𝑐)  . . . . . . . . . . . . . . . . . . . . 
Algorithm 19  ML-KEM.KeyGen() . . . . . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 20  ML-KEM.Encaps(ek)  . . . . . . . . . . . . . . . . . . . . . . . . . . 
. . . . . . . . . . . . . . . . . . . . . . . . 
Algorithm 21  ML-KEM.Decaps(dk, 𝑐) 

15 
39 
39 
46 

12 

8 
19 
20
20 
22 
22 
23 
23 
26 
26 
27 
27 
29 
30 
31 
32 
33 
34 
35 
37 
38 

vi 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

1. 

Introduction 

1.1  Purpose  and  Scope 

This standard specifies the Module-Lattice-Based Key-Encapsulation Mechanism (ML-KEM). A 
key-encapsulation mechanism (KEM) is a set of algorithms that can be used to establish a shared 
secret key between two parties communicating over a public channel. A KEM is a particular type 
of key establishment scheme. Other NIST-approved key establishment schemes are specified 
in NIST Special Publication (SP) 800-56A, Recommendation for Pair-Wise Key-Establishment 
Schemes Using Discrete Logarithm-Based Cryptography [2], and SP 800-56B, Recommendation 
for Pair-Wise Key Establishment Schemes Using Integer Factorization Cryptography [3]. 

The key establishment schemes specified in SP 800-56A and SP 800-56B are vulnerable to 
attacks that use sufficiently-capable quantum computers. ML-KEM is an approved alternative 
that is presently believed to be secure, even against adversaries in possession of a large-scale 
fault-tolerant quantum computer.  ML-KEM is derived from the round-three version of the 
CRYSTALS-KYBER KEM [4], a submission in the NIST Post-Quantum Cryptography Standardization 
project. For the differences between ML-KEM and CRYSTALS-KYBER, see Appendix C. 

This standard specifies the algorithms and parameter sets of the ML-KEM scheme.  It aims 
to provide sufficient information to implement ML-KEM in a manner that can pass validation 
(see https://csrc.nist.gov/projects/cryptographic- module- validation- program).  For 
general definitions and properties of KEMs, including requirements for the secure use of KEMs 
in applications, see SP 800-227 [1]. 

This standard specifies three parameter sets for ML-KEM that offer different trade-offs in security 
strength versus performance.  All three parameter sets of ML-KEM are approved to protect 
sensitive, non-classified communication systems of the U.S. Federal Government. 

1.2  Context 

Over the past several years, there has been steady progress toward building quantum computers. 
If large-scale quantum computers are realized, the security of many commonly used public-key 
cryptosystems will be at risk. This would include key-establishment schemes and digital signature 
schemes whose security depends on the difficulty of solving the integer factorization and discrete 
logarithm problems (both over finite fields and elliptic curves). As a result, in 2016, NIST initiated 
a public Post-Quantum Cryptography (PQC) Standardization process to select quantum-resistant 
public-key cryptographic algorithms. A total of 82 candidate algorithms were submitted to NIST 
for consideration. 

After three rounds of evaluation and analysis, NIST selected the first four algorithms for stan-
dardization. These algorithms are intended to protect sensitive U.S. Government information 
well into the foreseeable future, including after the advent of cryptographically-relevant quan-
tum computers. This standard specifies a variant of the selected algorithm CRYSTALS-KYBER, a 
lattice-based key-encapsulation mechanism (KEM) designed by Peter Schwabe, Roberto Avanzi, 
Joppe Bos, Léo Ducas, Eike Kiltz, Tancrède Lepoint, Vadim Lyubashevsky, John Schanck, Gregor 
Seiler, Damien Stehlé, and Jintai Ding  [4]. Throughout this standard, the KEM specified here will 
be referred to as ML-KEM, as it is based on the Module Learning With Errors assumption. 

1 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

2.  Terms,  Acronyms,  and  Notation 

2.1  Terms  and  Definitions 

approved 

FIPS-approved and/or NIST-recommended. An algorithm or technique 
that is either 1) specified in a FIPS or NIST recommendation, 2) adopted 
in a FIPS or NIST recommendation, or 3) specified in a list of NIST-approved 
security functions. 

(KEM) ciphertext 

A bit string that is produced by encapsulation and used as an input to 
decapsulation. 

cryptographic 
module 

decapsulation 

decapsulation key 

decryption key 

destroy 

encapsulation 

encapsulation key 

The set of hardware, software, and/or firmware that implements ap-
proved cryptographic functions (including key generation) that are con-
tained within the cryptographic boundary of the module. 

The process of applying the Decaps algorithm of a KEM. This algorithm 
accepts a KEM ciphertext and the decapsulation key as input and pro-
duces a shared secret key as output. 

A cryptographic key produced by a KEM during key generation and used 
during the decapsulation process. The decapsulation key must be kept 
private and must be destroyed after it is no longer needed. (See Section 
3.3.) 

A cryptographic key that is used with a PKE in order to decrypt cipher-
texts into plaintexts. The decryption key must be kept private and must 
be destroyed after it is no longer needed. 

An action applied to a key or other piece of secret data. After a piece 
of secret data is destroyed, no information about its value can be re-
covered. 

The process of applying the Encaps algorithm of a KEM. This algorithm 
accepts the encapsulation key as input, requires private randomness, 
and produces a shared secret key and an associated ciphertext as out-
put. 

A cryptographic key produced by a KEM during key generation and used 
during the encapsulation process. The encapsulation key can be made 
public. (See Section 3.3.) 

encryption key 

A cryptographic key that is used with a PKE in order to encrypt plaintexts 
into ciphertexts. The encryption key can be made public. 

equivalent process 

Two processes are equivalent if the same output is produced when the 
same values are input to each process (either as input parameters, as 
values made available during the process, or both). 

fresh random value  An output that was produced by a random bit generator and has not 

been previously used. 

2 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

hash function 

A function on bit strings in which the length of the output is fixed. 
Approved hash functions (such as those specified in FIPS 180 [5] and 
FIPS 202 [6]) are designed to satisfy the following properties: 

1.  (One-way) It is computationally infeasible to find any input that 

maps to any new pre-specified output. 

2.  (Collision-resistant) It is computationally infeasible to find any two 

distinct inputs that map to the same output. 

input checking 

Examination of a potential input to an algorithm for the purpose of 
determining whether it conforms to certain requirements. 

key 

A bit string that is used in conjunction with a cryptographic algorithm, 
such as the encapsulation and decapsulation keys (of a KEM), the shared 
secret key (produced by a KEM), and the encryption and decryption 
keys (of a PKE). (See Section 3.3.) 

key-encapsulation 
mechanism (KEM) 

A set of three cryptographic algorithms (KeyGen, Encaps, and Decaps) 
that can be used by two parties to establish a shared secret key over a 
public channel. 

key establishment 

A procedure that results in secret keying material that is shared among 
different parties. 

key pair 

little-endian 

party 

pseudorandom 

A set of two keys with the property that one key can be made public 
while the other key must be kept private. In this standard, this could 
refer to either the (encapsulation key, decapsulation key) key pair of a 
KEM or the (encryption key, decryption key) key pair of a PKE. 

The property of a byte string having its bytes positioned in order of 
increasing significance.  In particular, the leftmost (first) byte is the 
least significant, and the rightmost (last) byte is the most significant. 
The term “little-endian” may also be applied in the same manner to 
bit strings (e.g.,  the 8-bit string 11010001 corresponds to the byte 
20 + 21 + 23 + 27  = 139). 

An individual person, organization, device, or process. In this specifica-
tion, there are two parties (e.g., Party A and Party B, or Alice and Bob) 
who jointly perform the key establishment process using a KEM. 

A process (or data produced by a process) is said to be pseudorandom 
when the outcome is deterministic yet also appears random as long 
as the internal action of the process is hidden from observation. For 
cryptographic purposes, “effectively random” means “computationally 
indistinguishable from random within the limits of the intended security 
strength.” 

public channel 

A communication channel between two parties. Such a channel can be 
observed and possibly also corrupted by third parties. 

3 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

public-key 
encryption scheme 
(PKE) 

A set of three cryptographic algorithms (KeyGen, Encrypt, and Decrypt) 
that can be used by two parties to send secret data over a public channel. 
Also known as an asymmetric encryption scheme. 

shared secret 

shared secret key 

A secret value that has been computed during a key-establishment 
scheme, is known by both participants, and is used as input to a key-
derivation method to produce keying material. 

A shared secret that can be used directly as a cryptographic key in 
symmetric-key cryptography. It does not require additional key deriva-
tion. The shared secret key must be kept private and must be destroyed 
when no longer needed. 

security category 

A number associated with the security strength of a post-quantum 
cryptographic algorithm, as specified by NIST (see [7]). 

security strength 

A number associated with the amount of work (i.e., the number of op-
erations) that is required to break a cryptographic algorithm or system. 

shall 

should 

Used to indicate a requirement of this standard. 

Used to indicate a strong recommendation but not a requirement of 
this standard. Ignoring the recommendation could lead to undesirable 
results. 

2.2  Acronyms 

AES 

CBD 

FIPS 

KEM 

LWE 

MLWE 

NIST 

NISTIR 

NTT 

PKE 

PQC 

PRF 

RBG 

SHA 

Advanced Encryption Standard 

Centered Binomial Distribution 

Federal Information Processing Standard 

Key-Encapsulation Mechanism 

Learning with Errors 

Module Learning with Errors 

National Institute of Standards and Technology 

NIST Interagency or Internal Report 

Number-Theoretic Transform 

Public-Key Encryption 

Post-Quantum Cryptography 

Pseudorandom Function 

Random Bit Generator 

Secure Hash Algorithm 

4 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

SHAKE 

Secure Hash Algorithm KECCAK 

SP 

XOF 

Special Publication 

Extendable-Output Function 

2.3  Mathematical  Symbols 

𝑛 

𝑞 

𝜁 

𝔹 

ℚ 

ℤ 

ℤ𝑚 

ℤ𝑛
𝑚 

𝑅𝑞 

𝑇𝑞 

D𝜂(𝑅𝑞) 

𝑆∗ 

𝑆𝑘 

𝑓𝑗 
𝑓  ̂

Denotes the integer 256 throughout this document. 

Denotes the prime integer 3329 = 28 ⋅ 13 + 1 throughout this document. 

Denotes the integer 17, which is a primitive 𝑛-th root of unity modulo 𝑞. 

The set {0, 1, … , 255} of unsigned 8-bit integers (bytes). 

The set of rational numbers. 

The set of integers. 

The ring of integers modulo 𝑚 (i.e., the set {0, 1, … , 𝑚 − 1} equipped with 
the operations of addition and multiplication modulo 𝑚.) 

The set of 𝑛-tuples over ℤ𝑚  equipped with ℤ𝑚-module structure. As a data 
type, this is the set of length-𝑛 arrays whose entries are in ℤ𝑚. 
The ring ℤ𝑞[𝑋]/(𝑋𝑛 + 1) consisting of polynomials of the form 𝑓 = 𝑓0 + 
𝑓1𝑋 + ⋯ + 𝑓255𝑋255 , where 𝑓𝑗  ∈ ℤ𝑞  for all 𝑗. The ring operations are addi-
tion and multiplication modulo 𝑋𝑛 + 1. 

The image of 𝑅𝑞  under the number-theoretic transform. Its elements are 
called “NTT representations” of polynomials in 𝑅𝑞. (See Section 4.3.) 
A certain distribution of polynomials in 𝑅𝑞  with small coefficients, from 
which noise is sampled. The distribution is parameterized by 𝜂 ∈ {2, 3}. (See 
Section 4.2.2.) 

If 𝑆 is a set, this denotes the set of finite-length tuples (or arrays) of elements 
from the set 𝑆, including the empty tuple (or empty array). 

If 𝑆 is a set, this denotes the set of 𝑘-tuples (or length-𝑘 arrays) of elements 
from the set 𝑆. 
The coefficient of 𝑋𝑗  of a polynomial 𝑓 = 𝑓0 + 𝑓1𝑋 + ⋯ + 𝑓255𝑋255 ∈ 𝑅𝑞. 

The element of 𝑇𝑞  that is equal to the NTT representation of a polynomial
𝑓 ∈ 𝑅𝑞. (See Sections 2.4.4 and 4.3.) 

𝐯𝑇 , 𝐀𝑇 

The transpose of a row or column vector 𝐯. In general, the transpose of a 
matrix 𝐀. 

5 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

∘ 

×𝑇𝑞 

𝐴‖𝐵 

𝐵[𝑖] 

Denotes linear-algebraic composition with coefficients in 𝑅𝑞  or 𝑇𝑞  (e.g., 
𝐀 ∘ 𝐯 denotes the vector resulting from applying matrix 𝐀 to vector 𝐯). (See 
Section 2.4.7.) 

Denotes the operation on coefficient arrays that implements product in the 
ring 𝑇𝑞. (See Sections 2.4.5 and 4.3.1.) 
The concatenation of two arrays or bit strings 𝐴 and 𝐵. 

The entry at index 𝑖 in the array 𝐵. All arrays have indices that begin at zero. 

𝐵[𝑘 ∶ 𝑚] 

The subarray (𝐵[𝑘], 𝐵[𝑘 + 1], … , 𝐵[𝑚 − 1]) of the array 𝐵. 

|𝐵| 

⌈𝑥⌉ 

⌊𝑥⌋ 

⌈𝑥⌋ 

∶= 

If 𝐵 is a number, this denotes the absolute value of 𝐵. If 𝐵 is an array, this 
denotes its length. 

The ceiling of 𝑥 (i.e., the smallest integer greater than or equal to 𝑥). 

The floor of 𝑥 (i.e., the largest integer less than or equal to 𝑥). 

The rounding of 𝑥 to the nearest integer. If 𝑥 = 𝑦 + 1/2 for some 𝑦 ∈ ℤ, then 
⌈𝑥⌋ = 𝑦 + 1. 

Denotes that the left-hand side is defined to be the expression on the right-
hand side. 

𝑟 mod 𝑚 

The unique integer 𝑟′  in {0, 1, … , 𝑚 − 1} such that 𝑚 divides 𝑟 − 𝑟′ . 

BitRev7(𝑟) 

𝑠 ← 𝑥 

Bit reversal of a seven-bit integer 𝑟. Specifically, if 𝑟 = 𝑟0 + 2𝑟1 + 4𝑟2 + ⋯ + 
64𝑟6  with 𝑟𝑖  ∈ {0, 1}, then BitRev7(𝑟) = 𝑟6 + 2𝑟5 + 4𝑟4 + ⋯ + 64𝑟0. 
In pseudocode, this notation means that the variable 𝑠 is assigned the value 
of the expression 𝑥. 

$

𝑠 ←

− 𝔹ℓ 

In pseudocode, this notation means that the variable 𝑠 is assigned the value 
of an array of ℓ random bytes. The bytes must be freshly generated using 
randomness from an approved RBG. (See Section 3.3.) 

⊥ 

A symbol indicating failure or the lack of output from an algorithm. 

2.4 

Interpreting  the  Pseudocode 

This section outlines the conventions of the pseudocode used to describe the algorithms in 
this standard.  All algorithms are understood to have access to two global integer constants: 
𝑛 = 256 and 𝑞 = 3329.  There are also five global integer variables:  𝑘, 𝜂1, 𝜂2, 𝑑𝑢, and 𝑑𝑣.  All 
other variables are local. The five global variables are set to particular values when a parameter 
set is selected (see Section 8). 

When algorithms in this specification invoke other algorithms as subroutines, all arguments (i.e., 
inputs) are passed by value. In other words, a copy of the inputs is created, and the subroutine 
is invoked with the copy. There is no “passing by reference.” 

Pseudocode assignments are performed using the symbol “←.”  For example, the statement 
𝑧 ← 𝑦 means that the variable 𝑧 is assigned the value of variable 𝑦. Pseudocode comparisons 

6 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

are performed via the symbol “==.” For example, the expression 𝑥 == 𝑤 is a boolean value 
that is TRUE if and only if the variables 𝑥 and 𝑤 have the same value. 

In regular text (i.e., outside of the pseudocode), a different convention is applied. There, the 
“=” symbol is used both for assigning values and for comparisons, in keeping with standard 
mathematical notation. When emphasis is needed, assignments will be made with “∶=” instead. 

Variables will always have a valid value that is appropriate to their data type, with two exceptions: 

1.  The outputs of a random bit generator (RBG) have the byte array data type but are also 
allowed to have the special value NULL. This value indicates that randomness generation 
failed. This can only occur in ML-KEM.KeyGen and ML-KEM.Encaps. 

2.  The outputs of ML-KEM.KeyGen and ML-KEM.Encaps have the byte array data type but 
are also allowed to have the special value ⊥. When ML-KEM.KeyGen or ML-KEM.Encaps 
return the value ⊥, this indicates that the algorithm failed due to a failure of randomness 
generation. 

2.4.1  Data  Types 

For variables that represent the input or output of an algorithm, the data type (e.g., bit, byte, 
array of bits) will be explicitly described at the start of the algorithm. For most local variables 
in the pseudocode, the data type is easily deduced from context.  For all other variables, the 
data type will be declared in a comment.  In a single algorithm, the data type of a variable is 
determined the first time that the variable is used and will not be changed. Variable names can 
and will be reused across different algorithms, including with different data types. 

In addition to standard atomic data types (e.g., bits, bytes) and data structures (e.g., arrays), 
integers modulo 𝑚 (i.e., elements of ℤ𝑚) will also be used as an abstract data type. It is implicit 
that reduction modulo 𝑚 takes place whenever an assignment is made to a variable in ℤ𝑚. For 
example, for 𝑧 ∈ ℤ𝑚  and integers 𝑥 and 𝑦, the statement 

𝑧 ← 𝑥 + 𝑦 

(2.1) 

means that 𝑧 is assigned the value 𝑥 + 𝑦 mod 𝑚.  The pseudocode is agnostic regarding how 
an integer modulo 𝑚 is represented in actual implementations or how modular reduction is 
computed. 

2.4.2  Loop  Syntax 

The pseudocode will make use of both “while” and “for” loops.  The “while” syntax is self-
explanatory. In the case of “for” loops, the syntax will be in the style of the programming language 
C. Two simple examples are given in Algorithm 1. The standard mathematical expression (e.g., 
∑𝑛

(𝑖 + 3)) will be used for simple summations instead of a “for” loop. 

𝑖←1

2.4.3  Arithmetic  With  Arrays  of  Integers 

This standard makes extensive use of arrays of integers modulo 𝑚 (i.e., elements of ℤℓ 
𝑚). In a 
typical case, the relevant values are 𝑚 = 𝑞 = 3329 and ℓ = 𝑛 = 256. Arithmetic with arrays in 

7 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Algorithm 1 ForExample() 
Performs two simple “for” loops. 

𝐴[𝑖] ← 𝑖 

1:  for (𝑖 ← 0; 𝑖 < 10; 𝑖 ++) 
2: 
3:  end for 
𝑗 ← 0
4: 
5:  for (𝑘 ← 256; 𝑘 > 1; 𝑘 ← 𝑘/2) 
6: 
7: 
8:  end for 

𝐵[𝑗] ← 𝑘 
𝑗 ← 𝑗 + 1 

▷ 𝐴 is an integer array of length 10 
▷ 𝐴 now has the value (0, 1, 2, 3, 4, 5, 6, 7, 8, 9) 

▷ 𝐵 is an integer array of length 8 

▷ 𝐵 now has the value (256, 128, 64, 32, 16, 8, 4, 2) 

ℤℓ  will
𝑚 

 be done as 

follows. Let

 𝑎 ∈ ℤ and𝑚 

 𝑋, 𝑌  ∈ ℤℓ  .𝑚  The statements 

𝑍 ← 𝑎 ⋅ 𝑋 
𝑊 ← 𝑋 + 𝑌 

(2.2) 
(2.3) 

will result in two arrays 𝑍, 𝑊 ∈ ℤℓ
𝑚, with the property that 𝑍[𝑖] = 𝑎 ⋅ 𝑋[𝑖] and 𝑊 [𝑖] = 𝑋[𝑖] + 𝑌 [𝑖] 
for all 𝑖. Multiplication of arrays in ℤℓ
𝑚  will only be meaningful when 𝑚 = 𝑞 and ℓ = 𝑛 = 256, in 
which case it corresponds to multiplication in a particular ring. This operation will be described 
in (2.8). 

2.4.4  Representations  of  Algebraic  Objects 

An essential operation in ML-KEM is the number-theoretic transform (NTT), which maps a poly-
nomial 𝑓 in a certain ring 𝑅𝑞 to its “NTT representation” 𝑓 in̂  an isomorphic ring 𝑇𝑞. The rings 𝑅𝑞 
and 𝑇𝑞  and the NTT are discussed in detail in Section 4.3. This standard will represent elements 
of 𝑅𝑞  and 𝑇𝑞  in pseudocode using arrays of integers modulo 𝑞 as follows. 
An element 𝑓 of 𝑅𝑞  is a polynomial of the form 

𝑓 = 𝑓0 + 𝑓1𝑋 + ⋯ + 𝑓255𝑋255  ∈ 𝑅𝑞 

and will be represented in pseudocode by the array 

(𝑓0, 𝑓1, … , 𝑓255) ∈ ℤ256,

𝑞 

(2.4) 

(2.5) 

whose entries contain the coefficients of 𝑓. Overloading notation, the array in (2.5) will also be 
denoted by 𝑓. The 𝑖-th entry of the array 𝑓 will thus contain the 𝑖-th coefficient of the polynomial 
𝑓 (i.e., 𝑓[𝑖] = 𝑓𝑖). 
An element (sometimes called “NTT representation”) 𝑔 ̂ of 𝑇𝑞  is a tuple of 128 polynomials, each 
of degree at most one. Specifically, 

𝑔 = (   

𝑔0,0 + 𝑔0,1𝑋, 𝑔1,0 + 𝑔1,1𝑋, … , 𝑔127,0 + 

  𝑋) ∈ 𝑇𝑞 . 
𝑔127,1

(2.6)

8 

 
   
 
 
 
 
̂
̂
̂
̂
̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Such an algebraic object will be represented in pseudocode by the array 

(  
𝑔0,0, 𝑔0,1, 𝑔1,0, 

𝑔1,1, … ,   

𝑔127,0, 

𝑔127,1) ∈ ℤ256
𝑞 

. 

(2.7)

Overloading notation, the array in (2.7) will also be denoted by 𝑔.̂  In this case, the mapping 
for 𝑖 ∈ {0, 1, … , 127}.
between array entries and coefficients is 

𝑔𝑖,0̂  and 𝑔[2𝑖 + 1] = 𝑔𝑖,1̂ 

𝑔[2𝑖] = 

̂ 

̂ 

Converting between a polynomial 𝑓 ∈ 𝑅𝑞  and its NTT representation 𝑓 ∈ 𝑇𝑞  will be done via the 
algorithms NTT (Algorithm 9) and NTT−1  (Algorithm 10).  These algorithms act on arrays of 
coefficients, as described above, and satisfy 𝑓 ̂= NTT(𝑓) and 𝑓 = NTT−1(𝑓)̂ . 

2.4.5  Arithmetic  With  Polynomials  and  NTT  Representations 

The algebraic operations of addition and scalar multiplication in 𝑅𝑞  and 𝑇𝑞  are done coordinate-
wise.  For example, if 𝑎 ∈ ℤ𝑞  and 𝑓  ∈ 𝑅𝑞, the 𝑖-th coefficient of the polynomial 𝑎 ⋅ 𝑓  ∈ 𝑅𝑞  is 
equal to 𝑎 ⋅ 𝑓𝑖  mod 𝑞. In pseudocode, elements of both 𝑅𝑞 and 𝑇𝑞 are represented by coefficient 
arrays (i.e., elements of ℤ256
). The algebraic operations of addition and scalar multiplication are 
thus performed by addition and scalar multiplication of the corresponding coefficient arrays, 
as in (2.3) and (2.2).  For example, the addition of two NTT representations in pseudocode is 
performed by a statement of the form  ̂ 

ℎ ← 𝑓 + 𝑔, where ℎ, 𝑓, 𝑔 ∈ ℤ256

are coefficient arrays. 

𝑞 

̂ 

̂ 

𝑞 

The algebraic operations of multiplication in 𝑅𝑞  and 𝑇𝑞  are treated as follows.  For efficiency 
reasons, multiplication in 𝑅𝑞  will not be used. The algebraic meaning of multiplication in 𝑇𝑞  is 
discussed in Section 4.3.1. In pseudocode, it will be performed by the algorithm MultiplyNTTs 
(Algorithm 11). Specifically, if 
are a pair of arrays (each representing the NTT of some
polynomial), then 

𝑓, 𝑔 ∈ ℤ256
̂ 

𝑞 

̂ 
ℎ ← 𝑓 ×𝑇𝑞 

𝑔  

means 

ℎ ← MultiplyNTTs(

𝑓, 𝑔) . 

(2.8)

The result is an array  ̂ 

ℎ ∈ ℤ256 
.

𝑞 

2.4.6  Matrices  and  Vectors 

In addition to arrays of integers modulo 𝑞, the pseudocode will also make use of arrays whose 
𝑞  )3  will be a length-
entries are themselves elements of ℤ256
. For example, an element 𝐯 ∈ (ℤ256
𝑞 
three array whose entries 𝐯[0], 𝐯[1], and 𝐯[2] are themselves elements of ℤ256
(i.e., arrays). One 
𝑞 
can think of each of these entries as representing a polynomial in 𝑅𝑞  so that 𝐯 itself represents 
an element of the module 𝑅𝑞
When arrays are used to represent matrices and vectors whose entries are elements of 𝑅𝑞, they 
will be denoted with bold letters (e.g., 𝐯 for vectors and 𝐀 for matrices). When arrays are used 
to represent matrices and vectors whose entries are elements of 𝑇𝑞, they will be denoted with a 
“hat” (e.g., 𝐯̂ and 𝐀̂ ). Unless an explicit transpose operation is performed, it is understood that 
vectors are column vectors. One can then view vectors as the special case of matrices with only 
one column. 

3 . 

Converting between matrices over 𝑅𝑞  and matrices over 𝑇𝑞  will be done coordinate-wise. For 

9 

 
 
 
 
̂
̂
̂
̂
̂
̂
̂
̂
̂
̂
̂
 
 
 
̂
̂
̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

example, if 𝐯 ∈ (ℤ256

𝑞 

)𝑘 

, then the statement 

̂ 𝐯 ← NTT(𝐯) 

(2.9)

)𝑘  such that 𝐯[𝑖]

̂  = NTT(𝐯[𝑖]) for all 𝑖. This involves running NTT a total 

will result in 
of 𝑘 times. 

𝐯 ∈ (ℤ256
̂ 

𝑞 

2.4.7  Arithmetic  With  Matrices  and  Vectors 

The following describes how to perform arithmetic with matrices over 𝑅𝑞  and 𝑇𝑞  with vectors as 
a special case. 

Addition and scalar multiplication are performed coordinate-wise, so the addition of matrices 
over 𝑅𝑞  and 𝑇𝑞  is straightforward. In the case of 𝑇𝑞, scalar multiplication is done via (2.8). For 
̂ 
example, if 𝑓 ∈ ℤ256  and 

𝐮, 𝐯 ∈ (ℤ256)𝑘 

, then

̂ 

𝑞 

𝑞 

𝑓 ⋅  𝐮 
𝐰 ← 
𝐳 ← 𝐮 + 
  𝐯 
̂ 

(2.10) 
(2.11)

̂ 

̂ 

̂ 

̂ 

̂ 

̂ 

𝐰, 𝐳 ∈ (ℤ256 

𝑞  )𝑘  satisfying 𝐰[𝑖] = ̂ 

will result in 
𝐮[𝑖] and 𝐳[𝑖] = 𝐮[𝑖] + 𝐯[𝑖] for all 𝑖. Here, the 
𝑓 ×𝑇𝑞 
multiplication and addition of individual entries are performed using the appropriate arithmetic 
for coefficient arrays of elements of 𝑇𝑞  (i.e., as in (2.3)). 
It will also be necessary to multiply matrices with entries in 𝑇𝑞, which is done by using standard 
matrix multiplication with the base-case multiplication (i.e., multiplication of individual entries) 
being multiplication in 𝑇𝑞. If 𝐀̂  and 𝐁̂  are two matrices with entries in 𝑇𝑞, their matrix product 
will be denoted 𝐀 ∘̂  𝐁.̂  Some example pseudocode statements involving matrix multiplication 
are given in (2.12), (2.13), and (2.14). In these examples, 𝐀̂  is a 𝑘 × 𝑘 matrix, while 𝐮̂ and 𝐯̂ are 
vectors of length 𝑘. All three of these objects are represented in pseudocode by arrays: a 𝑘 × 𝑘 
array for 𝐀̂  and length-𝑘 arrays for 𝐮̂ and 𝐯̂. The entries of 𝐀̂ , 𝐮̂, and 𝐯̂ are elements of ℤ256
. In 
(2.12) and (2.13), the pseudocode statement on the left produces a new length-𝑘 vector whose 
entries are specified on the right. In (2.14), the pseudocode statement on the left computes a 
𝑧 of ℤ256
. 
dot product. The result is in the base ring (i.e., 𝑇𝑞) and is represented by an element 

𝑞 

̂ 

𝑞 

𝐰 ←  𝐀 ∘ 𝐮 

𝐲 ←  𝐀⊺ ∘ 𝐮 

𝑧 ← 

𝐮⊺ ∘ 𝐯 

𝑘−1 
𝐰[𝑖] = ∑ 𝐀[𝑖, 𝑗] ×𝑇𝑞
𝑗=0 
𝑘−1 

𝐮[𝑗] 

(2.12) 

𝐲[𝑖] = ∑ 𝐀[𝑗, 𝑖] ×𝑇𝑞

𝐮[𝑗] 

𝑗=0 
𝑘−1 

𝑧 = ∑ 𝐮[𝑗] ×𝑇𝑞

𝐯[𝑗] 

𝑗=0 

(2.13) 

(2.14)

The multiplication ×𝑇𝑞 
in (2.8). 

of individual entries above is performed using MultiplyNTTs, as described 

10 

̂
̂
̂
̂
̂
̂
̂
 
 
̂
̂
̂
̂
̂
̂
 
 
̂
̂
̂
̂
̂
̂
 
 
 
 
̂
̂
̂
̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

2.4.8  Applying  Algorithms  to  Arrays,  Examples 

In the previous examples, arithmetic over ℤ𝑚  was extended to arithmetic with arrays over ℤ𝑚 
and then further extended to arithmetic with matrices whose entries are themselves arrays over 
ℤ𝑚. Similarly, algorithms defined with a given data type as input will be applied to arrays and 
matrices over that data type. When the pseudocode invokes such an algorithm on an array or 
matrix input, it is implied that the algorithm is invoked repeatedly and applied to each entry of 
the input. 

For example, consider the function Compress𝑑 
on an array input 𝐹  ∈ ℤ256

𝑞  with the statement 

∶ ℤ𝑞 → ℤ2𝑑  defined in Section 4. It can be invoked 

𝐾 ← Compress𝑑(𝐹 ) . 

(2.15) 

The result will be an array 𝐾 ∈ ℤ256 such that 𝐾[𝑖] = Compress𝑑(𝐹 [𝑖]) for every 𝑖 ∈ {0, 1, … , 255}. 
2𝑑 
The computation (2.15) involves 

running the Compress algorithm 256 times. 

For a second example, consider the algorithm NTT defined in Section 4.3. It takes an array 𝑓 ∈ 
(representing an element of 𝑅𝑞) as input and outputs another array 𝑓 ∈ ℤ256 (representing 
ℤ256 
𝑞 
𝑞  )𝑘  (representing an 
an element of 𝑇𝑞).  If the NTT algorithm is invoked on a vector 𝐬 ∈ (ℤ256
element of 𝑅𝑞

𝑘) with the pseudocode statement 

𝑞 

̂ 𝐬 ← NTT(𝐬) , 

(2.16)

𝐬 ∈ (ℤ256 
the result is a vector  ̂ 
𝐬̂ represents an element of 𝑇𝑞
times. 

𝑞  )𝑘 such that 𝐬[𝑖] = NTT(𝐬[𝑖]) for all 𝑖 ∈ {0, 1, … , 𝑘 − 1}. The vector 
𝑘 . The computation (2.16) involves running the NTT algorithm 𝑘 

̂ 

For a third example, consider line 2 of K-PKE.Encrypt in Section 5.2: 

̂𝐭 ← ByteDecode12(ekPKE[0 ∶ 384𝑘]) . 

(2.17) 

ByteDecode12 
an integer array in ℤ256
byte array ekPKE  and results in  ̂ 
each subarray ekPKE[384 ⋅ 𝑗, 384 ⋅ (𝑗 + 1)], and will result in an integer array 𝐭[𝑗] ∈ ℤ256
̂ 
𝐭[𝑗] = ByteDecode12(ekPKE 

is defined to receive a byte array of length 32 ⋅ 12 = 384 as input and produce 
as output.  The computation (2.17) is run on the first 384𝑘 bytes of 
will thus be applied 𝑘 times, once for 
such that 

[384 ⋅ 𝑗, 384 ⋅ (𝑗 + 1)]) for each 𝑗 from 0 to 𝑘 − 1.

. ByteDecode12 

𝐭 ∈ (ℤ256 

𝑞  )𝑘 

𝑞 

𝑞 

11 

̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

3.  Overview  of  the  ML-KEM  Scheme 

This section gives a high-level overview of the ML-KEM scheme. 

3.1  Key-Encapsulation  Mechanisms 

The following is a high-level overview of key-encapsulation mechanisms (KEMs). For details, see 
SP 800-227 [1]. 

A KEM is a cryptographic scheme that, under certain conditions, can be used to establish a shared 
secret key between two communicating parties.  This shared secret key can then be used for 
symmetric-key cryptography. 

A KEM consists of three algorithms and a collection of parameter sets. The three algorithms are: 

1.  A probabilistic key generation algorithm denoted by KeyGen 

2.  A probabilistic ”encapsulation” algorithm denoted by Encaps 

3.  A deterministic ”decapsulation” algorithm denoted by Decaps 

The collection of parameter sets is used to select a trade-off between security and efficiency. 
Each parameter set in the collection is a list of specific (typically numerical) values, one for each 
parameter required by the three algorithms. 

Alice 

Bob 

𝐾′ 

𝐾 

Figure 1. A simple view of key establishment using a KEM 

12 

decapsulation keyDecapsciphertextAlice’s copy of the shared secret keyEncapsBob’s copy of the shared secret keyencapsulation keyKeyGenFIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

In the typical application, a KEM is used to establish a shared secret key between two parties 
(here referred to as Alice and Bob) as described in Figure 1. Alice begins by running KeyGen in 
order to generate a (public) encapsulation key and a (private) decapsulation key. Upon obtaining 
Alice’s encapsulation key, Bob runs the Encaps algorithm, which produces Bob’s copy 𝐾 of the 
shared secret key along with an associated ciphertext. Bob sends the ciphertext to Alice, and 
Alice completes the process by running the Decaps algorithm using her decapsulation key and 
the ciphertext. This final step produces Alice’s copy 𝐾′  of the shared secret key. 

After completing this process, Alice and Bob would like to conclude that their outputs satisfy 
𝐾′  = 𝐾 and that this value is a secure, random, shared secret key. However, these properties 
only hold if certain important conditions are satisfied, as discussed in SP 800-227 [1]. 

3.2  The  ML-KEM  Scheme 

ML-KEM is a key-encapsulation mechanism based on CRYSTALS-KYBER [4], a scheme that was 
initially described in [8]. The following is a brief and informal description of the computational 
assumption underlying ML-KEM and how the ML-KEM scheme is constructed. 

The computational assumption. The security of ML-KEM is based on the presumed hardness 
of the so-called Module Learning with Errors (MLWE) problem [9], which is a generalization of 
the Learning With Errors (LWE) problem introduced by Regev in 2005 [10]. The hardness of the 
MLWE problem is itself based on the presumed hardness of certain computational problems in 
module lattices [9]. This motivates the name of the scheme ML-KEM. 

In the LWE problem,  the input is a set of random “noisy” linear equations in some secret 
variables 𝑥 ∈ ℤ𝑛
𝑞 , and the task is to recover 𝑥. The noise in the equations is such that standard 
algorithms (e.g., Gaussian elimination) are intractable. The LWE problem naturally lends itself to 
cryptographic applications. For example, if 𝑥 is interpreted as a secret key, then one can encrypt 
a one-bit plaintext value by sampling either an approximately correct linear equation (if the 
plaintext is zero) or a far-from-correct linear equation (if the plaintext is one). Plausibly, only a 
party in possession of 𝑥 can distinguish these two cases. Encryption can then be delegated to 
another party by publishing a large collection of noisy linear equations, which can be combined 
appropriately by the encrypting party. The result is an asymmetric encryption scheme. 

The MLWE problem is similar to the LWE problem. An important difference is that, in MLWE, ℤ𝑛
𝑞 
𝑘 , which is constructed by taking the 𝑘-fold Cartesian product 
is replaced by a certain module 𝑅𝑞
of a certain polynomial ring 𝑅𝑞. In particular, the secret in the MLWE problem is an element 𝐱 of 
the module 𝑅𝑞

𝑘 . The ring 𝑅𝑞  is discussed in detail in Section 4.3. 

The ML-KEM construction. At a high level, the construction of the scheme ML-KEM proceeds in 
two steps. First, the ideas discussed previously are used to construct a public-key encryption (PKE) 
scheme from the MLWE problem. Second, this PKE scheme is converted into a key-encapsulation 
mechanism using the so-called Fujisaki-Okamoto (FO) transform [11, 12]. Due to certain prop-
erties of the FO transform, the resulting KEM provides security in a significantly more general 
attack model than the PKE scheme. As a result, ML-KEM is believed to satisfy so-called IND-CCA2 
security [1, 4, 13, 14]. 

13 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

The specification of the ML-KEM algorithms in this standard will follow the same pattern. Specif-
ically, this standard will first describe a public-key encryption scheme called K-PKE (in Section 5) 
and then use the algorithms of K-PKE as subroutines when describing the algorithms of ML-KEM 
(in Sections 6 and 7).  The cryptographic transformation from K-PKE to ML-KEM is crucial for 
achieving IND-CCA2 security. The scheme K-PKE is not IND-CCA2-secure and shall not be used as 
a stand-alone scheme (see Section 3.3). 

A notable feature of ML-KEM is the use of the number-theoretic transform (NTT). The NTT 
converts a polynomial 𝑓 ∈ 𝑅𝑞 to an alternative representation as a vector 𝑓 of linear polynomials. 
Working with NTT representations enables significantly faster multiplication of polynomials. 
Other operations (e.g., addition, rounding, and sampling) can be done in either representation. 

ML-KEM satisfies the essential KEM property of correctness.  This means that in the absence 
of corruption or interference, the process in Figure 1 will result in 𝐾′  = 𝐾 with overwhelming 
probability.  ML-KEM also comes with a proof of asymptotic theoretical security in a certain 
heuristic model [4]. Each of the parameter sets of ML-KEM comes with an associated security 
strength that was estimated based on current cryptanalysis (see Section 8 for details). 

Parameter sets and algorithms. Recall that a KEM consists of algorithms KeyGen, Encaps, and 
Decaps, along with a collection of parameter sets. In the case of ML-KEM, the three aforemen-
tioned algorithms are: 

1.  ML-KEM.KeyGen (Algorithm 19) 

2.  ML-KEM.Encaps (Algorithm 20) 

3.  ML-KEM.Decaps (Algorithm 21) 

These algorithms are described and discussed in detail in Section 7. 

ML-KEM comes equipped with three parameter sets: 

•  ML-KEM-512 (security category 1) 

•  ML-KEM-768 (security category 3) 

•  ML-KEM-1024 (security category 5) 

These parameter sets are described and discussed in detail in Section 8. The security categories 
1-5 are defined in SP 800-57, Part 1 [7]. Each parameter set assigns a particular numerical value 
to five integer variables: 𝑘, 𝜂1, 𝜂2, 𝑑𝑢, and 𝑑𝑣. The values of these variables in each parameter 
set are given in Table 2 of Section 8. In addition to these five variable parameters, there are also 
two constants: 𝑛 = 256 and 𝑞 = 3329. 

Decapsulation failures. Provided that all inputs are well-formed and randomness generation is 
successful, the key establishment procedure of ML-KEM will never explicitly fail, meaning that 
both ML-KEM.Encaps and ML-KEM.Decaps will each output a 256-bit value.  Moreover, if no 
corruption or interference is present, the two 256-bit values produced by ML-KEM.Encaps and 
ML-KEM.Decaps will be equal with overwhelming probability (i.e., 𝐾′ will equal 𝐾 in the process 
described in Figure 1). The event that 𝐾′  ≠ 𝐾 under these conditions is called a decapsulation 

14 

̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

failure. Formally, the decapsulation failure probability is defined to be the probability (conditioned 
on no RGB failures) that the process 

1. (ek, dk) ← ML-KEM.KeyGen() 
2. (𝑐, 𝐾) ← ML-KEM.Encaps(ek) 
3. 𝐾′  ← ML-KEM.
, 𝑐) 

Decaps dk

(

(3.1) 
(3.2) 
(3.3) 

results in 𝐾′  ≠ 𝐾.  The probability is taken over uniformly random seeds 𝑑, 𝑧 (sampled in 
ML-KEM.KeyGen) and 𝑚 (sampled in ML-KEM.Encaps) and under the heuristic assumption that 
hash functions and XOFs behave like uniformly random functions. The decapsulation failure rates 
for ML-KEM are listed in Table 1. For details, see Theorem 1 in [8] and the scripts in [15]. 

Table 1. Decapsulation failure rates for ML-KEM 

Parameter set  Decapsulation failure rate 
ML-KEM-512 
ML-KEM-768 
ML-KEM-1024 

2−138.8 
2−164.8 
2−174.8 

Terminology for keys.  A KEM involves three different types of keys:  encapsulation keys, de-
capsulation keys, and shared secret keys. ML-KEM is built on top of the component public-key 
encryption scheme K-PKE, which has two additional key types: encryption keys and decryption 
keys.  In the literature, encapsulation keys and encryption keys are sometimes referred to as 
“public keys,” while decapsulation keys and decryption keys are sometimes referred to as “pri-
vate keys.”  In order to reduce confusion, this standard will not use the terms “public key” or 
“private key.”  Instead, keys will be referred to only using the more specific terms, i.e., one of 
“encapsulation key”, “decapsulation key”, “encryption key”, “decryption key”, and “shared secret 
key”. 

3.3  Requirements  for  ML-KEM  Implementations 

This section describes several requirements for cryptographic modules that implement ML-KEM. 
Implementation requirements specific to particular algorithms will be described in later sections. 
Additional requirements,  including requirements for using ML-KEM in specific applications, 
are given in SP 800-227 [1].  While conforming implementations must adhere to all of these 
requirements, adherence does not guarantee that the result will be secure (see Point 13 in the 
announcement). 

K-PKE is only a component. The public-key encryption scheme K-PKE described in Section 5 shall 
not be used as a stand-alone cryptographic scheme. Instead, the algorithms that comprise K-PKE 
may only be used as subroutines in the algorithms of ML-KEM.  In particular, the algorithms 
K-PKE.KeyGen (Algorithm 13), K-PKE.Encrypt (Algorithm 14), and K-PKE.Decrypt (Algorithm 15) 
are not approved for use as a public-key encryption scheme. 

15 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Controlled access to internal functions. The key-encapsulation mechanism ML-KEM makes use 
of internal, “derandomized” functions ML-KEM.KeyGen_internal, ML-KEM.Encaps_internal, and 
ML-KEM.Decaps_internal, specified in Section 6. The interfaces for these functions should not 
be made available to applications other than for testing purposes. In particular, the sampling of 
random values required for key generation (as specified in ML-KEM.KeyGen) and encapsulation 
(as specified in ML-KEM.Encaps) shall be performed by the cryptographic module. 

Equivalent implementations. For every algorithm that is specified in this standard, a conforming 
implementation may replace the given set of steps with any mathematically equivalent set of 
steps. In other words, the specified algorithm may be replaced with a different procedure that 
produces the correct output for every input (where “input” includes the specified input as well 
as all parameter values and all randomness). 

Approved usage of the shared secret key. If randomness generation is successful, the values 
𝐾 and 𝐾′  returned by ML-KEM.Encaps and ML-KEM.Decaps, respectively, are always 256-bit 
values. Under appropriate conditions (see Sections 3.1 and 3.2, and SP 800-227 [1]), these values 
match (i.e., 𝐾′ = 𝐾) and can be used directly as a shared secret key for symmetric cryptography. 
If further key derivation is needed, the final symmetric keys shall be derived from this 256-bit 
shared secret key in an approved manner, as specified in SP 800-108 and SP 800-56C [16, 17]. 

As discussed in Section 3.2, ML-KEM is an IND-CCA2-secure KEM. However, a combined KEM 
that includes ML-KEM as a component might not meet IND-CCA2 security. Implementers should 
assess the security of any procedure in which the key derivation methods of SP 800-56C are 
applied to ML-KEM in combination with another key establishment procedure. More guidance 
regarding combined KEMs is given in SP 800-227 [1]. 

Randomness generation. Two algorithms in this standard require the generation of randomness 
as an internal step:  ML-KEM.KeyGen and ML-KEM.Encaps.  In pseudocode, this randomness 
generation is denoted by a statement of the form 𝑚 ←− 𝔹32 .  A fresh string of random bytes 
must be generated for every such invocation. These random bytes shall be generated using an 
approved RBG, as prescribed in SP 800-90A, SP 800-90B, and SP 800-90C [18, 19, 20]. Moreover, 
this RBG shall have a security strength of at least 128 bits for ML-KEM-512, at least 192 bits for 
ML-KEM-768, and at least 256 bits for ML-KEM-1024. 

$ 

Input checking.  The algorithms ML-KEM.Encaps and ML-KEM.Decaps require input checking. 
Implementers shall ensure that ML-KEM.Encaps and ML-KEM.Decaps are only executed on 
inputs that have been checked, as described in Section 7. 

Destruction of intermediate values.  Data used in intermediate computation steps of KEM 
algorithms could be used by an adversary to compromise security.  Therefore, implementers 
shall ensure that intermediate data is destroyed as soon as it is no longer needed. In particular, 
for ML-KEM.KeyGen, ML-KEM.Encaps, and ML-KEM.Decaps, only the designated output can be 
retained in memory after the algorithm terminates. All other data shall be destroyed prior to 

16 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

the algorithm terminating. 

There are two exceptions to this rule: 

1.  The seed (𝑑, 𝑧) generated in steps 1 and 2 of ML-KEM.KeyGen can be stored for later 
expansion using ML-KEM.KeyGen_internal.  As this seed can be used to compute the 
decapsulation key, it is sensitive data and shall be treated with the same safeguards as a 
decapsulation key (see SP 800-227 [1]). 

2.  The matrix 𝐀̂ generated in steps 3-7 of K-PKE.KeyGen (as a subroutine of ML-KEM.KeyGen) 
can be stored so that it need not be recomputed in later operations (e.g., ML-KEM.Decaps). 
The same matrix 𝐀̂  is also generated in steps 4-8 of K-PKE.Encrypt (as a subroutine of 
ML-KEM.Encaps or ML-KEM.Decaps); it can also then be stored. In either case, the matrix 
𝐀̂  is data that is easily computed from the public encapsulation key and thus does not 
require any special protections. 

No floating-point arithmetic. Implementations of ML-KEM shall not use floating-point arithmetic, 
as rounding errors in floating-point operations may lead to incorrect results in some cases. In 
all pseudocode in this standard in which division is performed (e.g., 𝑥/𝑦) and 𝑦 may not divide 
𝑥, either ⌊𝑥/𝑦⌋, ⌈𝑥/𝑦⌉, or ⌈𝑥/𝑦⌋ is used. All of these can be computed without floating-point 
arithmetic, as ordinary integer division 𝑥/𝑦 computes ⌊𝑥/𝑦⌋, and ⌈𝑥/𝑦⌉ = ⌊(𝑥 + 𝑦 − 1)/𝑦⌋ for 
non-negative integers 𝑥 and positive integers 𝑦. 

17 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

4.  Auxiliary  Algorithms 

4.1  Cryptographic  Functions 

The algorithms specified in this publication require the use of several cryptographic functions. 
Each function shall be instantiated by means of an approved hash function or an approved 
eXtendable-Output Function (XOF), as prescribed below. The relevant hash functions and XOFs 
are described in detail in FIPS 202 [6]. They will be used as follows. 

SHA3-256 and SHA3-512 are hash functions with one variable-length input and one fixed-length 
output.  In this standard, invocations of these functions on an input 𝑀 will be denoted by 
SHA3-256(𝑀 ) and SHA3-512(𝑀 ), respectively. The inputs and outputs for both SHA3-256 and 
SHA3-512 are always byte arrays. 

SHAKE128 and SHAKE256 are XOFs with one variable-length input and one variable-length output. 
This standard will adhere to the following conventions [6]: 

•  The inputs and outputs for both SHAKE128 and SHAKE256 are always byte arrays. 

•  When invoking SHAKE128 or SHAKE256, desired output length is always specified in bits. 

For example, the expression 

𝑟 ∶= SHAKE128(𝑀 , 8 ⋅ 64) 

(4.1) 

implies that 𝑀 is an array of bytes and that 𝑟 is an array of 64 bytes. 

The aforementioned functions play several different roles in the algorithms specified in this 
standard and will only be invoked using the wrapper functions defined below. Importantly, these 
wrappers will avoid any potential “byte array” versus “bit-length” confusion by only working with 
bytes and byte array lengths. 

Pseudorandom function (PRF). The function PRF takes a parameter 𝜂 ∈ {2, 3}, one 32-byte 
input, and one 1-byte input. It produces one (64 ⋅ 𝜂)-byte output. It will be denoted by 

PRF ∶ {2, 3} × 𝔹32 × 𝔹 → 𝔹64𝜂 , 

and it shall be instantiated as 

PRF𝜂(𝑠, 𝑏) ∶= SHAKE256(𝑠‖𝑏, 8 ⋅ 64 ⋅ 𝜂) , 

(4.2) 

(4.3) 

where 𝜂 ∈ {2, 3}, 𝑠 ∈ 𝔹32 , and 𝑏 ∈ 𝔹.  Note that 𝜂 is only used to specify the desired output 
length and not to perform domain separation. 

Hash functions. The specification will also make use of three hash functions H, J and G, which 
are defined as follows. 

The functions H and J each take one variable-length input and produce one 32-byte output. They 
will be denoted by H ∶ 𝔹∗  → 𝔹32  and J ∶ 𝔹∗  → 𝔹32 , respectively, and shall be instantiated as 

H(𝑠) ∶= SHA3-256(𝑠) 

and 

J(𝑠) ∶= SHAKE256(𝑠, 8 ⋅ 32) 

(4.4) 

18 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

where 𝑠 ∈ 𝔹∗ . 

The function G takes one variable-length input and produces two 32-byte outputs.  It will be 
denoted by G ∶ 𝔹∗  → 𝔹32 × 𝔹32 . The two outputs of G will be denoted by (𝑎, 𝑏) ← G(𝑐), where 
𝑎, 𝑏 ∈ 𝔹32 , 𝑐 ∈ 𝔹∗ , and G(𝑐) = 𝑎‖𝑏. The function G shall be instantiated as 

G(𝑐) ∶= SHA3-512(𝑐) . 

(4.5) 

eXtendable-Output Function (XOF). This standard uses a XOF wrapper defined in terms of the 
incremental API for SHAKE128 in SP 800-185 [21]. This SHAKE128 API consists of three functions: 

•  ctx ← SHAKE128.Init() 

Initializes a XOF “context” ctx. 

•  ctx ← SHAKE128.Absorb(ctx, str) 

Injects data to be used in the “absorbing” phase of SHAKE128 and updates the context 
accordingly. 

•  (ctx, 𝐵) ← SHAKE128.Squeeze(ctx, 8 ⋅ 𝑧) 

Extracts 𝑧 output bytes produced during the “squeezing” phase of SHAKE128 and updates 
the context accordingly. 

While the above functions are constructed using the Keccak-𝑓 permutation rather than the XOF 
SHAKE128 directly, they are defined so that a single SHAKE128 call of the form 

output ← SHAKE128(str1‖ … ‖str𝑚, 8𝑏1 + … + 8𝑏ℓ) 

(4.6) 

is equivalent to performing Algorithm 2. This equivalence holds whether or not |str𝑖| and 𝑏𝑗  are 
multiples of the SHAKE128 block length. 

Algorithm 2 SHAKE128example(str1, … , str𝑚, 𝑏1, … , 𝑏ℓ) 
Performs a sequence of absorbing operations followed by a sequence of squeezing operations. 
Input: byte arrays str1, … , str𝑚. 
Input: positive integers 𝑏1, … , 𝑏ℓ. 
Output: a byte array of length ∑ℓ 
1:  ctx ← SHAKE128.Init() 
2:  for (𝑖 ← 1; 𝑖 ≤ 𝑚; 𝑖++) 
3: 
4:  end for 
5:  for (𝑗 ← 1; 𝑗 ≤ ℓ; 𝑗++) 
6: 
7:  end for 
8:  output ← out1‖ … ‖outℓ

(ctx, out𝑗) ← SHAKE128.Squeeze(ctx, 8 ⋅ 𝑏𝑗) 

ctx ← SHAKE128.Absorb(ctx, str𝑖) 

▷ return the concatenation of all the results 

▷ squeeze 𝑏𝑗-many bytes 

▷ absorb byte array str𝑖 

▷ initialize context 

𝑏𝑗. 

𝑗=1 

In this standard, the incremental API for SHAKE128 will only be invoked through a wrapper XOF, 

19 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

which is defined as follows. 

1.  XOF.Init() = SHAKE128.Init(). 

2.  XOF.Absorb(ctx, str) = SHAKE128.Absorb(ctx, str). 

3.  XOF.Squeeze(ctx, ℓ) = SHAKE128.Squeeze(ctx, 8 ⋅ ℓ). 

Note that XOF.Squeeze requires the input length to be specified in bytes. This is consistent with 
the convention that all wrapper functions treat inputs and outputs as byte arrays and measure 
the lengths of all such arrays in terms of bytes. 

4.2  General  Algorithms 

This section specifies a number of algorithms that will be used as subroutines in ML-KEM. 

4.2.1  Conversion  and  Compression  Algorithms 

This section specifies several algorithms for converting between bit arrays, byte arrays, and arrays 
of integers modulo 𝑚. It also specifies a certain operation for compressing integers modulo 𝑞, 
and the corresponding decompression operation. 

Algorithm 3 BitsToBytes(𝑏) 
Converts a bit array (of a length that is a multiple of eight) into an array of bytes. 
Input: bit array 𝑏 ∈ {0, 1}8⋅ℓ . 
Output: byte array 𝐵 ∈ 𝔹ℓ . 
1:  𝐵 ← (0, … , 0) 
2:  for (𝑖 ← 0; 𝑖 < 8ℓ; 𝑖++) 
3: 
4:  end for 
5:  return 𝐵 

𝐵 [⌊𝑖/8⌋] ← 𝐵 [⌊𝑖/8⌋] + 𝑏[𝑖] ⋅ 2𝑖 mod 8 

Algorithm 4 BytesToBits(𝐵) 
Performs the inverse of BitsToBytes, converting a byte array into a bit array. 
Input: byte array 𝐵 ∈ 𝔹ℓ . 
Output: bit array 𝑏 ∈ {0, 1}8⋅ℓ . 
1:  𝐶 ← 𝐵 
2:  for (𝑖 ← 0; 𝑖 < ℓ; 𝑖 ++) 
3: 
4: 
5: 
6: 
7:  end for 
8:  return 𝑏 

𝑏[8𝑖 + 𝑗] ← 𝐶[𝑖] mod 2 
𝐶[𝑖] ← ⌊𝐶[𝑖]/2⌋ 

for (𝑗 ← 0; 𝑗 < 8; 𝑗++) 

end for 

▷ copy 𝐵 into array 𝐶 ∈ 𝔹ℓ 

20 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Converting between bits and bytes. The algorithms BitsToBytes (Algorithm 3) and BytesToBits 
(Algorithm 4) convert between bit arrays and byte arrays. The inputs to BitsToBytes and the 
outputs of BytesToBits are bit arrays, with each segment of eight bits representing a byte in 
little-endian order. As an example, the 8-bit string 11010001 corresponds to the byte 20 + 21 + 
23 + 27  = 139. 

Compression and decompression. Recall that 𝑞 = 3329, and that the bit length of 𝑞 is 12. For 
𝑑 < 12, define 

Compress𝑑 

∶ ℤ𝑞 ⟶ ℤ2𝑑 

𝑥 ⟼ ⌈(2𝑑/𝑞) ⋅ 𝑥⌋ mod 2𝑑 . 

Decompress𝑑 

∶ ℤ2𝑑  ⟶ ℤ𝑞 

𝑦 ⟼ ⌈(𝑞/2𝑑) ⋅ 𝑦⌋ . 

(4.7) 

(4.8) 

The input and output types of these functions are integers modulo 𝑚 (see Section 2.4.1). Division 
and rounding in the computation of these functions are performed in the set of rational numbers. 
Floating-point computations shall not be used. 

The Compress and Decompress algorithms satisfy two important properties. First, decompression 
followed by compression preserves the input.  That is, Compress𝑑(Decompress𝑑(𝑦)) = 𝑦 for 
all 𝑦 ∈ ℤ2𝑑  and all 𝑑 < 12.  Second, if 𝑑 is large (i.e., close to 12), compression followed by 
decompression does not significantly alter the value. 

Encoding and decoding. The algorithms ByteEncode (Algorithm 5) and ByteDecode (Algorithm 
6) will be used for conversion between integers modulo 𝑚 and bytes. The algorithm ByteEncode 
converts an array of 𝑛 = 256 integers modulo 𝑚 into a corresponding array of bytes. ByteDecode 
performs the inverse operation, converting an array of bytes into an array of integers modulo 𝑚. 
Specifying the modulus 𝑚 is done as described below. 

For the following description, it is convenient to view ByteDecode and ByteEncode as converting 
between integers and bits. The conversion between bits and bytes is straightforward and done 
using BitsToBytes and BytesToBits. The valid range of values for the parameter 𝑑 is 1 ≤ 𝑑 ≤ 12. 
Bit arrays are divided into 𝑑-bit segments. The operations are performed in two different ways, 
depending on the value of 𝑑: 

•  For 𝑑 satisfying 1 ≤ 𝑑 ≤ 11, the conversion is one-to-one.  ByteDecode𝑑 
𝑑-bit segment of its input into one integer modulo 2𝑑 , while ByteEncode𝑑 
inverse operation. 

converts each 
performs the 

•  For 𝑑 = 12, ByteDecode12 

produces integers modulo 𝑞 as output, while ByteEncode12 
receives integers modulo 𝑞 as input.  Specifically,  ByteDecode12 
converts each 12-bit 
segment of its input into an integer modulo 212  = 4096 and then reduces the result 
modulo 𝑞. This is no longer a one-to-one operation. Indeed, some 12-bit segments could 
correspond to an integer greater than 𝑞 − 1 = 3328 but less than 4096.  However, this 
and 
cannot occur for arrays produced by ByteEncode12

. These aspects of ByteDecode12 

21 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

ByteEncode12 
key in Section 7. 

will be important when considering checking of the ML-KEM encapsulation 

Algorithm 5 ByteEncode (𝐹 ) 

𝑑

𝑎 ← 𝐹 [𝑖] 
for (𝑗 ← 0; 𝑗 < 𝑑; 𝑗++) 

Encodes an array of 𝑑-bit integers into a byte array for 1 ≤ 𝑑 ≤ 12. 
Input: integer array 𝐹  ∈ ℤ256 , where 𝑚 = 2𝑑  if 𝑑 < 12, and 𝑚 = 𝑞 if 𝑑 = 12.
𝑚 
Output: byte array 𝐵 ∈ 𝔹32𝑑 . 
1:  for (𝑖 ← 0; 𝑖 < 256; 𝑖 ++) 
2: 
3: 
4: 
5: 
6: 
7:  end for 
8:  𝐵 ← BitsToBytes(𝑏) 
9:  return 𝐵 

𝑏[𝑖 ⋅ 𝑑 + 𝑗] ← 𝑎 mod 2 
𝑎 ← (𝑎 − 𝑏[𝑖 ⋅ 𝑑 + 𝑗])/2 

end for 

▷ 𝑎 ∈ ℤ𝑚 

▷ 𝑏 ∈ {0, 1}256⋅𝑑 
▷ note 𝑎 − 𝑏[𝑖 ⋅ 𝑑 + 𝑗] is always even 

Algorithm 6 ByteDecode (𝐵) 

𝑑

Decodes a byte array into an array of 𝑑-bit integers for 1 ≤ 𝑑 ≤ 12. 
Input: byte array 𝐵 ∈ 𝔹32𝑑 . 
Output: integer array 𝐹  ∈ ℤ256 , where 𝑚 = 2𝑑  if 𝑑 < 12 and 𝑚 = 𝑞 if 𝑑 = 12.
1:  𝑏 ← BytesToBits(𝐵) 
2:  for (𝑖 ← 0; 𝑖 < 256; 𝑖 ++) 
𝐹 [𝑖] ← ∑𝑑−1 
3: 
𝑗←0 
4:  end for 
5:  return 𝐹 

𝑏[𝑖 ⋅ 𝑑 + 𝑗] ⋅ 2𝑗  mod 𝑚 

𝑚 

4.2.2  Sampling  Algorithms 

The algorithms of ML-KEM require two sampling subroutines that are specified in Algorithms 7 
and 8. Both of these algorithms can be used to convert a stream of uniformly random bytes into 
a sample from some desired distribution. In this standard, these algorithms will be invoked with 
a stream of pseudorandom bytes as the input. It follows that the output will then be a sample 
from a distribution that is computationally indistinguishable from the desired distribution. 

Uniform sampling of NTT representations. The algorithm SampleNTT (Algorithm 7) converts 
a seed together with two indexing bytes into a polynomial in the NTT domain.  If the seed is 
uniformly random, the resulting polynomial will be drawn from a distribution that is computa-
tionally indistinguishable from the uniform distribution on 𝑇𝑞. The output of SampleNTT is an 
array in ℤ256
that contains the coefficients of the sampled element of 𝑇𝑞 (see Section 2.4.4). See 
Appendix B for a note on (optionally) safely bounding the algorithm’s while-loop iterations. 

𝑞 

22 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

𝑎 ∈ ℤ256 
.
̂ 

▷ input the given byte array into XOF 

▷ a 32-byte seed along with two indices 
▷ the coefficients of the NTT of a polynomial 

Algorithm 7 SampleNTT(𝐵) 
Takes a 32-byte seed and two indices as input and outputs a pseudorandom element of 𝑇𝑞. 
Input: byte array 𝐵 ∈ 𝔹34 . 
Output: array 
𝑞 
1:  ctx ← XOF.Init() 
2:  ctx ← XOF.Absorb(ctx, 𝐵) 
3:  𝑗 ← 0 
4:  while 𝑗 < 256 do 
5: 
6: 
7: 
8: 
9: 
10: 
11: 
12: 
13: 
14: 
end if 
15: 
16:  end while 
17:  return 𝑎̂ 

(ctx, 𝐶) ← XOF.Squeeze(ctx, 3) 
𝑑1  ← 𝐶[0] + 256 ⋅ (𝐶[1] mod 16) 
𝑑2 ← ⌊𝐶[1]/16⌋ + 16 ⋅ 𝐶[2] 
if 𝑑1  < 𝑞 then 
𝑎[𝑗] ← 𝑑1 
̂ 
𝑗 ← 𝑗 + 1 

end if 
if 𝑑2  < 𝑞 and 𝑗 < 256 then 

▷ get a fresh 3-byte array 𝐶 from XOF 
▷ 0 ≤ 𝑑1 < 212 
▷ 0 ≤ 𝑑2 < 212 

̂𝑎[𝑗] ← 𝑑2 
𝑗 ← 𝑗 + 1 

▷ 𝑎 ∈ ℤ256
̂ 

𝑞 

Sampling from the centered binomial distribution. ML-KEM makes use of a special distribution 
D𝜂(𝑅𝑞) of polynomials in 𝑅𝑞  with small coefficients. Such polynomials are sometimes referred 
to as “errors” or “noise.” The distribution is parameterized by an integer 𝜂 ∈ {2, 3}. To sample a 
polynomial from D𝜂(𝑅𝑞), each of its coefficients is independently sampled from a certain cen-
tered binomial distribution (CBD) on ℤ𝑞. The algorithm SamplePolyCBD (Algorithm 8) samples 
the coefficient array of a polynomial 𝑓 ∈ 𝑅𝑞  according to the distribution D𝜂(𝑅𝑞), provided that 

Algorithm 8 SamplePolyCBD (𝐵) 

𝜂

Takes a seed as input and outputs a pseudorandom sample from the distribution D𝜂(𝑅𝑞). 
Input: byte array 𝐵 ∈ 𝔹64𝜂 . 
Output: array 𝑓 ∈ ℤ256
. 
1:  𝑏 ← BytesToBits(𝐵) 
2:  for (𝑖 ← 0; 𝑖 < 256; 𝑖 ++) 
𝑏[2𝑖𝜂 + 𝑗] 
3: 

▷ the coefficients of the sampled polynomial 

▷ 0 ≤ 𝑥 ≤ 𝜂 

𝑞 

𝑥 ← ∑𝜂−1 
𝑗←0 
𝑦 ← ∑𝜂−1 
𝑗←0 
𝑓[𝑖] ← 𝑥 − 𝑦 mod 𝑞 

𝑏[2𝑖𝜂 + 𝜂 + 𝑗] 

4: 

5: 
6:  end for 
7:  return 𝑓 

▷ 0 ≤ 𝑦 ≤ 𝜂 
▷ 0 ≤ 𝑓[𝑖] ≤ 𝜂 or 𝑞 − 𝜂 ≤ 𝑓[𝑖] ≤ 𝑞 − 1 

23 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

its input is a stream of uniformly random bytes. 

4.3  The  Number-Theoretic  Transform 

The Number-Theoretic Transform (NTT) can be viewed as a specialized, exact version of the 
discrete Fourier transform. In the case of ML-KEM, the NTT is used to improve the efficiency of 
multiplication in the ring 𝑅𝑞. Recall that 𝑅𝑞  is the ring ℤ𝑞[𝑋]/(𝑋𝑛 + 1) of polynomials of the 
form 𝑓 = 𝑓0 + 𝑓1𝑋 + ⋯ + 𝑓255𝑋255  (where 𝑓𝑗  ∈ ℤ𝑞  for all 𝑗), with the ring operations defined 
by arithmetic modulo 𝑋𝑛 + 1. 

The ring 𝑅𝑞  is isomorphic to another ring 𝑇𝑞, which is a direct sum of 128 quadratic extensions 
of ℤ𝑞. The NTT is a computationally efficient isomorphism between these two rings. When a 
polynomial 𝑓 ∈ 𝑅𝑞  is input, the NTT outputs an element 
𝑓 ∶= NTT(𝑓) of the ring 𝑇𝑞, where 𝑓 is 
called the “NTT representation” of 𝑓. The isomorphism property implies that 

𝑓 ×𝑅𝑞 

𝑔 = NTT−1(𝑓 × 

𝑔), 

𝑇𝑞 

(4.9) 

and ×𝑇𝑞 

where ×𝑅𝑞 
denote multiplication in 𝑅𝑞  and 𝑇𝑞, respectively. Moreover, since 𝑇𝑞  is a 
product of 128 rings that each consist of polynomials of degree at most one, the operation ×𝑇𝑞 
is much more efficient than the operation ×𝑅𝑞 
. For these reasons, the NTT is considered to be 
an integral part of ML-KEM and not merely an optimization. 

As the rings 𝑅𝑞  and 𝑇𝑞  have a vector space structure over ℤ𝑞, the most natural abstract data 
type to represent elements from either of these rings is ℤ𝑛
𝑞 . For this reason, the choice of data 
structure for the inputs and outputs of NTT and NTT−1  are length-𝑛 arrays of integers modulo 
𝑞.  These arrays are understood to represent elements of 𝑇𝑞  or 𝑅𝑞, respectively (see Section 
2.4.4). Algorithms 9 and 10 describe an efficient means of computing NTT and NTT−1  in place. 
However, to clarify the distinction between the algebraic objects before and after the conversion, 
the algorithms are written with explicit inputs and outputs. This is consistent with this standard’s 
convention that all inputs are passed by copy. 

The mathematical structure of the NTT. In ML-KEM, 𝑞 is the prime 3329 = 28 ⋅ 13 + 1, and 
𝑛 = 256. There are 128 primitive 256-th roots of unity and no primitive 512-th roots of unity in 
ℤ𝑞. Note that 𝜁 = 17 ∈ ℤ𝑞  is a primitive 256-th root of unity modulo 𝑞. Thus, 𝜁128  ≡ −1. 
Define BitRev7(𝑖) to be the integer represented by bit-reversing the unsigned 7-bit value that 
corresponds to the input integer 𝑖 ∈ {0, … , 127}. 

The polynomial 𝑋256 + 1 factors into 128 polynomials of degree 2 modulo 𝑞 as follows: 

𝑋256 + 1 = 

(𝑋2 − 𝜁2BitRev7(𝑖)+1) . 

127 
∏ 
𝑖=0 

(4.10) 

Therefore, 𝑅𝑞  ∶= ℤ𝑞[𝑋]/(𝑋256 + 1) is isomorphic to a direct sum of 128 quadratic extension 

24 

̂
̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

fields of ℤ𝑞, denoted 𝑇𝑞. Specifically, this ring has the structure 

127 

𝑇𝑞  ∶= ⨁ ℤ𝑞[𝑋]/ (𝑋2 − 𝜁2BitRev7(𝑖)+1) . 

𝑖=0 

(4.11) 

Thus, the NTT representation 𝑓 ∈ 𝑇𝑞  of a polynomial 𝑓 ∈ 𝑅𝑞  is the vector that consists of the 
corresponding residues of degree at most one: 

̂𝑓 ∶= (𝑓 mod (𝑋2 − 𝜁2BitRev7(0)+1), … , 𝑓 mod (𝑋2 − 𝜁2BitRev7(127)+1)) . 

(4.12) 

As discussed in Section 2.4.4, the algorithms in this standard represent 𝑓 ̂as an array of 256 
integers modulo 𝑞. Specifically, 

𝑓 mod (𝑋2 − 𝜁2BitRev7(𝑖)+1) = 

 𝑓[2𝑖] + 𝑓[2𝑖 + 1]𝑋, 

(4.13) 

for 𝑖 from 0 to 127. 

The ML-KEM NTT algorithms. An algorithm for the ML-KEM NTT is described in Algorithm 9. An 
algorithm for the inverse operation (NTT−1) is described in Algorithm 10. These two algorithms 
will be used to transform elements of 𝑅𝑞  to elements of 𝑇𝑞  (using NTT) and vice versa (using 
NTT−1). In addition, as discussed in Section 2.4.8, these algorithms represent the coordinate-
wise transformation of structures over those rings. Specifically, they map matrices/vectors with 
entries in 𝑅𝑞  to matrices/vectors with entries in 𝑇𝑞  (using NTT) and vice versa (using NTT−1). 
The values 𝜁BitRev7(𝑖)  mod 𝑞 for 𝑖 = 1, … , 127 used in line 5 of Algorithm 9 and line 5 of Algorithm 
10 may be precomputed into an array. This array is given in Appendix A. 

25 

̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Algorithm 9 NTT(𝑓) 

Computes the NTT representation 𝑓 of the given polynomial 𝑓 ∈ 𝑅𝑞. 
Input: array 𝑓 ∈ ℤ256
. 
Output: array 𝑓 ∈ ℤ256

▷ the coefficients of the input polynomial 
▷ the coefficients of the NTT of the input polynomial 

. 

𝑞 

𝑞 

▷ will compute in place on a copy of input array 

for (start ← 0; start < 256; start ← start + 2 ⋅ len) 

1:  𝑓 ← 𝑓 
2:  𝑖 ← 1 
3:  for (len ← 128; len ≥ 2; len ← len/2) 
4: 
5: 
6: 
7: 
8: 

zeta ← 𝜁BitRev7(𝑖)  mod 𝑞 
𝑖 ← 𝑖 + 1 
for (𝑗 ← start; 𝑗 < start + len; 𝑗++)

𝑡 ← zeta ⋅ 𝑓[𝑗 + len]
𝑓[𝑗 + len] ← 𝑓[𝑗] − 𝑡 
𝑓[𝑗] ← 𝑓[𝑗] + 𝑡 

9: 

end for 

10: 
11: 
12: 
13:  end for 
14:  return 𝑓 ̂ 

end for 

▷ steps 8-10 done modulo 𝑞 

Algorithm 10 NTT (𝑓) ̂

−1

𝑞 

𝑞 

. 

▷ will compute in place on a copy of input array 

▷ the coefficients of input NTT representation 
▷ the coefficients of the inverse NTT of the input 

for (start ← 0; start < 256; start ← start + 2 ⋅ len) 

Computes the polynomial 𝑓 ∈ 𝑅𝑞  that corresponds to the given NTT representation 𝑓 ∈ 𝑇𝑞. 
Input: array 𝑓 ∈ ℤ256
. 
Output: array 𝑓 ∈ ℤ256
1:  𝑓 ← 𝑓 ̂ 
2:  𝑖 ← 127 
3:  for (len ← 2; len ≤ 128; len ← 2 ⋅ len) 
4: 
5: 
6: 
7: 
8: 
9: 
10: 
11: 
12: 
13:  end for 
14:  𝑓 ← 𝑓 ⋅ 3303 mod 𝑞 
15:  return 𝑓 

zeta ← 𝜁BitRev7(𝑖)  mod 𝑞 
𝑖 ← 𝑖 − 1 
for (𝑗 ← start; 𝑗 < start + len; 𝑗++) 

𝑡 ← 𝑓[𝑗] 
𝑓[𝑗] ← 𝑡 + 𝑓[𝑗 + len]
𝑓[𝑗 + len] ← zeta ⋅ (𝑓[𝑗 + len] − 𝑡) 

▷ multiply every entry by 3303 ≡ 128−1  mod 𝑞 

▷ steps 9-10 done modulo 𝑞 

end for 

end for 

26 

̂
̂
̂
̂
̂
̂
̂
̂
 
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

4.3.1  Multiplication  in  the  NTT  Domain 

The addition and scalar multiplication of elements of 𝑇𝑞  can be done using the corresponding 
coordinate-wise arithmetic operations on the coefficient arrays (see Section 2.4.5). This section 
describes how to do the remaining ring operation (i.e., multiplication in 𝑇𝑞). 

Recall from (4.12) that 𝑓 ∈ 𝑇𝑞  is a vector of 128 degree-one residues modulo quadratic polyno-
mials. Algebraically, multiplication in the ring 𝑇𝑞  consists of independent multiplication in each 
of the 128 coordinates with respect to the quadratic modulus of that coordinate. Specifically, 
the 𝑖-th coordinate in 𝑇𝑞  of the product ℎ̂ = 𝑓 ×̂ 

𝑔 ̂ is determined by the calculation 

𝑇𝑞

ℎ[2𝑖] + ℎ[2𝑖 + 1]𝑋 = (𝑓[2𝑖] + 𝑓[2𝑖 + 1]𝑋)(𝑔[2𝑖] + 𝑔[2𝑖 + 1]𝑋) mod (𝑋2 − 𝜁2BitRev7(𝑖)+1). 

̂ 

(4.14) 

̂ 

Algorithm 11 MultiplyNTTs(𝑓, 𝑔) 
Computes the product (in the ring 𝑇𝑞) of two NTT representations. 
Input: Two arrays 𝑓 ∈ ℤ256
and 𝑔 ∈ ℤ256 
. 
̂ 
𝑞 
Output: An array ℎ ∈ ℤ256
. 
1:  for (𝑖 ← 0; 𝑖 < 128; 𝑖 ++)
̂ 
2: 
3:  end for 
4:  return ℎ̂ 

𝑞 

𝑞 

(ℎ[2𝑖], ℎ[2𝑖 + 1]) ← BaseCaseMultiply(𝑓[2𝑖], 𝑓[2𝑖 + 1], 𝑔[2𝑖], 𝑔[2𝑖 + 1], 𝜁2BitRev7(𝑖)+1) 

▷ the coefficients of two NTT representations 
▷ the coefficients of the product of the inputs 

Thus, one can compute the product of two elements of 𝑇𝑞 using the algorithm MultiplyNTTs (Al-
gorithm 11), which uses BaseCaseMultiply (Algorithm 12) as a subroutine. The values 𝜁2BitRev7(𝑖)+1 
used in Algorithm 11 may be precomputed and stored in an array (see Appendix A). MultiplyNTTs 
also enables linear-algebraic arithmetic with matrices and vectors whose entries are in 𝑇𝑞  (see 
Section 2.4.7). 

Algorithm 12 BaseCaseMultiply(𝑎0, 𝑎1, 𝑏0, 𝑏1, 𝛾) 
Computes the product of two degree-one polynomials with respect to a quadratic modulus. 
Input: 𝑎0, 𝑎1, 𝑏0, 𝑏1  ∈ ℤ𝑞. 
Input: 𝛾 ∈ ℤ𝑞. 
Output: 𝑐0, 𝑐1  ∈ ℤ𝑞. 
1:  𝑐0 ← 𝑎0 ⋅ 𝑏0 + 𝑎1 ⋅ 𝑏1 ⋅ 𝛾 
2:  𝑐1  ← 𝑎0 ⋅ 𝑏1 + 𝑎1 ⋅ 𝑏0 
3:  return (𝑐0, 𝑐1) 

▷ the coefficients of 𝑎0  + 𝑎1𝑋 and 𝑏0  + 𝑏1𝑋 
▷ the modulus is 𝑋2 − 𝛾 
▷ the coefficients of the product of the two polynomials 
▷ steps 1-2 done modulo 𝑞 

27 

̂
̂
̂
̂
̂
̂
̂
̂
̂
̂
̂
̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

5.  The  K-PKE  Component  Scheme 

This section describes the component scheme K-PKE. As discussed in Section 3.3, K-PKE is not 
approved for use in a stand-alone fashion. It serves only as a collection of subroutines for use in 
the algorithms of the approved scheme ML-KEM, as described in Section 7. 

K-PKE consists of three algorithms: 

1.  Key generation (K-PKE.KeyGen) 

2.  Encryption (K-PKE.Encrypt) 

3.  Decryption (K-PKE.Decrypt) 

When K-PKE is instantiated as part of ML-KEM, K-PKE inherits the parameter set selected for 
ML-KEM. Each parameter set specifies numerical values for each parameter. While 𝑛 is always 
256 and 𝑞 is always 3329, the values of the remaining parameters 𝑘, 𝜂1, 𝜂2, 𝑑𝑢, and 𝑑𝑣  vary 
among the three parameter sets. Parameters and parameter sets are described in Section 8. 

The algorithms in this section do not perform any input checking because they are only invoked as 
subroutines of the main ML-KEM algorithms. The algorithms of ML-KEM themselves do perform 
input checking as needed. 

Each of the algorithms of K-PKE is accompanied by a brief, informal description in text.  For 
simplicity, this description is written in terms of vectors and matrices whose entries are elements 
of 𝑅𝑞. In the actual algorithm, most of the computations occur in the NTT domain in order to 
improve the efficiency of multiplication. The relevant vectors and matrices will then have entries 
in 𝑇𝑞. Linear-algebraic arithmetic with such vectors and matrices (e.g., line 18 of K-PKE.KeyGen) 
is performed as described in Sections 2.4.7 and 4.3.1. The encryption and decryption keys of 
K-PKE are also stored in the NTT form. 

5.1  K-PKE  Key  Generation 

The key generation algorithm K-PKE.KeyGen of K-PKE (Algorithm 13) receives a seed as input and 
outputs an encryption key ekPKE and a decryption key dkPKE. As is typically the case for public-key 
encryption, the encryption key can be made public, while the decryption key and the randomness 
must remain private. Indeed, the encryption key of K-PKE will serve as the encapsulation key of 
ML-KEM (see ML-KEM.KeyGen below) and can thus be made public. Meanwhile, the decryption 
key and seed of K-PKE.KeyGen must remain private as they can be used to perform decapsulation 
in ML-KEM. 
The matrix 𝐀̂  generated in steps 3-7 of K-PKE.KeyGen can be stored, as specified in Section 3.3. 
This allows later operations to use 𝐀̂  directly rather than re-expanding it from the public seed 𝜌. 

Informal description. The decryption key of K-PKE.KeyGen is a length-𝑘 vector 𝐬 of elements 
𝑘). Roughly speaking, 𝐬 is a set of secret variables, while the encryption key is 
of 𝑅𝑞  (i.e., 𝐬 ∈ 𝑅𝑞
a collection of “noisy” linear equations (𝐀, 𝐀𝐬 + 𝐞) in the secret variables 𝐬. The rows of the 
matrix 𝐀 form the equation coefficients. This matrix is generated pseudorandomly using XOF 
with only a seed stored in the encryption key. The secret 𝐬 and the “noise” 𝐞 are sampled from 

28 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

. 

∈ 𝔹384𝑘+32
∈ 𝔹384𝑘 
. 

Algorithm 13 K-PKE.KeyGen(𝑑) 
Uses randomness to generate an encryption key and a corresponding decryption key. 
Input: randomness 𝑑 ∈ 𝔹32 . 
Output: encryption key ekPKE 
Output: decryption key dkPKE 
1:  (𝜌, 𝜎) ← G(𝑑‖𝑘) 
2:  𝑁 ← 0 
3:  for (𝑖 ← 0; 𝑖 < 𝑘; 𝑖 ++) 
4: 
5: 
6: 
7:  end for 
8:  for (𝑖 ← 0; 𝑖 < 𝑘; 𝑖 ++) 
9: 

𝐀[𝑖, 𝑗] ← SampleNTT(𝜌‖𝑗‖𝑖) 

for (𝑗 ← 0; 𝑗 < 𝑘; 𝑗++)

end for 

(𝜎, 𝑁 )) 

̂ 

(PRF𝜂1

▷ generate matrix 𝐀∈ (ℤ256

)𝑘×𝑘 

𝑞 

▷ 𝑗 and 𝑖 are bytes 33 and 34 of the input 

▷ generate 𝐬 ∈ (ℤ256

)𝑘 
▷ 𝐬[𝑖] ∈ ℤ256  sampled from CBD

𝑞 

𝑞 

▷ expand 32+1 bytes to two pseudorandom 32-byte seeds1 

𝐬[𝑖] ← SamplePolyCBD𝜂1
𝑁 ← 𝑁 + 1 

10: 
11:  end for 
12:  for (𝑖 ← 0; 𝑖 < 𝑘; 𝑖 ++) 
13: 

𝐞[𝑖] ← SamplePolyCBD𝜂1
𝑁 ← 𝑁 + 1 

14: 
15:  end for 
𝐬 ← NTT(𝐬) 
16: 
𝐞 ← NTT(𝐞)
17: 
18:  𝐭 ← 𝐀 ∘ 𝐬 +̂  𝐞̂ 
19:  ekPKE  ← ByteEncode12(𝐭)‖𝜌 
20:  dkPKE  ← ByteEncode12(𝐬)̂ 
21:  return (ekPKE, dkPKE) 

(PRF𝜂1

(𝜎, 𝑁 )) 

▷ generate 𝐞 ∈ (ℤ256

)𝑘 
▷ 𝐞[𝑖] ∈ ℤ256  sampled from CBD

𝑞 

𝑞 

▷ run NTT 𝑘 times (once for each coordinate of 𝐬)
▷ run NTT 𝑘 times
▷ noisy linear system in NTT domain 
𝑘 times, then append 𝐀-seed 
𝑘 times 
▷ run ByteEncode12 

▷ run ByteEncode12 

centered binomial distributions using randomness expanded from another seed 𝜎 via PRF. Once 
𝐀, 𝐬, and 𝐞 are generated, the computation of the final part 𝐭 = 𝐀𝐬 + 𝐞 of the encryption key 
takes place. The results are appropriately encoded into byte arrays and output. 

In K-PKE.KeyGen, the choice of parameter set affects the length of the secret 𝐬 (via the parameter 
𝑘) and, as a consequence, the sizes of the noise vector 𝐞 and the pseudorandom matrix 𝐀. The 
choice of parameter set also affects the noise distribution (via the parameter 𝜂1) used to sample 
the entries of 𝐬 and 𝐞. 

5.2  K-PKE  Encryption 

The encryption algorithm K-PKE.Encrypt of K-PKE (Algorithm 14) takes an encryption key ekPKE, 
a 32-byte plaintext 𝑚, and randomness 𝑟 as input and produces a single output: a ciphertext 𝑐. 

1Byte 33 of the input to G is the module dimension 𝑘 ∈ {2, 3, 4} ⊂ 𝔹. This is included to establish domain separation 
between the three parameter sets. For implementations that use the seed in place of the private key, this ensures 
that the expansion will produce an unrelated key if the seed is mistakenly expanded using a parameter set other 
than the originally intended one. 

29 

̂
̂
̂
̂
̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

The matrix 𝐀̂  generated in steps 4-8 of K-PKE.Encrypt can be stored, as specified in Section 3.3. 
This allows later operations to use 𝐀̂  directly rather than re-expanding it from the public seed 𝜌. 

∈ 𝔹384𝑘+32

Algorithm 14 K-PKE.Encrypt(ekPKE, 𝑚, 𝑟) 
Uses the encryption key to encrypt a plaintext message using the randomness 𝑟. 
Input: encryption key ekPKE 
. 
Input: message 𝑚 ∈ 𝔹32 . 
Input: randomness 𝑟 ∈ 𝔹32 . 
Output: ciphertext 𝑐 ∈ 𝔹32(𝑑𝑢𝑘+𝑑𝑣). 
1:  𝑁 ← 0
̂ 
2:  𝐭 ← ByteDecode12(ekPKE[0 ∶ 384𝑘])  ▷ run ByteDecode12 
3:  𝜌 ← ekPKE[384𝑘 ∶ 384𝑘 + 32] 
4:  for (𝑖 ← 0; 𝑖 < 𝑘; 𝑖 ++) 
5: 
6: 
7: 
8:  end for 
9:  for (𝑖 ← 0; 𝑖 < 𝑘; 𝑖 ++) 
10: 

▷ re-generate matrix 𝐀 ∈ (ℤ256

𝐀[𝑖, 𝑗] ← SampleNTT(𝜌‖𝑗‖𝑖) 

for (𝑗 ← 0; 𝑗 < 𝑘; 𝑗++)

end for 

(𝑟, 𝑁 )) 

𝐲[𝑖] ← SamplePolyCBD𝜂1
𝑁 ← 𝑁 + 1 

(PRF𝜂1

𝑘 times to decode 𝐭 ∈ (ℤ256 

)𝑘 
𝑞 
▷ extract 32-byte seed from 
ekPKE
𝑞  )𝑘×𝑘  sampled in Alg. 13 

̂ 

▷ 𝑗 and 𝑖 are bytes 33 and 34 of the input 

▷ generate 𝐲 ∈ (ℤ256

)𝑘 
▷ 𝐲[𝑖] ∈ ℤ256  sampled from CBD

𝑞 

𝑞 

(PRF𝜂2

(𝑟, 𝑁 )) 

▷ generate 𝐞𝟏  ∈ (ℤ256
𝑞 

)𝑘 
▷ 𝐞𝟏[𝑖] ∈ ℤ256  sampled from CBD

𝑞 

11: 
12:  end for 
13:  for (𝑖 ← 0; 𝑖 < 𝑘; 𝑖 ++) 
14: 

(𝑟, 𝑁 )) 

𝐞𝟏[𝑖] ← SamplePolyCBD𝜂2
𝑁 ← 𝑁 + 1 

(PRF𝜂2

𝐲 ← NTT(𝐲) 

15: 
16:  end for 
17:  𝑒2  ← SamplePolyCBD𝜂2
18: 
19:  𝐮 ← NTT−1(𝐀̂ ⊺ ∘ 𝐲) + 𝐞𝟏 
20:  𝜇 ← Decompress1(ByteDecode1(𝑚)) 
21:  𝑣 ← NTT−1(𝐭⊺̂  ∘ 𝐲) + 𝑒2 + 𝜇 
̂ 
22:  𝑐1  ← ByteEncode𝑑𝑢
23:  𝑐2  ← ByteEncode𝑑𝑣
24:  return 𝑐 ← (𝑐1‖𝑐2) 

(Compress𝑑𝑢
(Compress𝑑𝑣

(𝐮)) 
(𝑣)) 

̂ 

▷ sample 𝑒2  ∈ ℤ256  from CBD
▷ run NTT 𝑘 times
▷ run NTT−1  𝑘 times

𝑞 

▷ encode plaintext 𝑚 into polynomial 𝑣 
𝑘 times 

and Compress𝑑𝑢 

▷ run ByteEncode𝑑𝑢 

Informal description.  The algorithm K-PKE.Encrypt begins by extracting the vector 𝐭 and the 
seed from the encryption key. The seed is then expanded to re-generate the matrix 𝐀 in the same 
manner as was done in K-PKE.KeyGen. If 𝐭 and 𝐀 are derived correctly from an encryption key 
produced by K-PKE.KeyGen, then they are equal to their corresponding values in K-PKE.KeyGen. 

Recall from the description of key generation that the pair (𝐀, 𝐭 = 𝐀𝐬 + 𝐞) can be thought of as 
a system of noisy linear equations in the secret variables 𝐬. One can generate an additional noisy 
linear equation in the same secret variables — without knowing 𝐬 — by picking a random linear 

30 

̂
̂
̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

combination of the noisy equations in the system (𝐀, 𝐭). One can then encode information in the 
“constant term” (i.e., the entry that is a linear combination of entries of 𝐭) of such a combined 
equation. This information can then be deciphered by a party in possession of 𝐬. For example, 
one could encode a single bit by deciding whether or not to significantly alter the constant term, 
thus making either a nearly correct equation that corresponds to the decrypted bit value of 0 or a 
far-from-correct equation that corresponds to the decrypted bit value of 1. In the case of K-PKE, 
the constant term is a polynomial with 256 coefficients, so one can encode more information: 
one bit in each coefficient. 

𝑘 
𝑘  and noise terms 𝐞𝟏  ∈ 𝑅𝑞
To this end, K-PKE.Encrypt proceeds by generating a vector 𝐲 ∈ 𝑅𝑞
and 𝑒2  ∈ 𝑅𝑞, all of which are sampled from the centered binomial distribution using pseudoran-
domness expanded via PRF from the input randomness 𝑟. One then computes the “new noisy 
equation,” which is (up to some details) (𝐀⊺𝐲 + 𝐞1, 𝐭⊺𝐲 + 𝑒2). An appropriate encoding 𝜇 of 
the input message 𝑚 is then added to the latter term in the pair. Finally, the resulting pair (𝐮, 𝑣) 
is compressed, serialized into a byte array, and output as the ciphertext. 

5.3  K-PKE  Decryption 

The decryption algorithm K-PKE.Decrypt of K-PKE (Algorithm 15) takes a decryption key dkPKE 
and a ciphertext 𝑐 as input, requires no randomness, and outputs a plaintext 𝑚. 

Informal description. The algorithm K-PKE.Decrypt begins by recovering a pair (𝐮′, 𝑣′) from the 
ciphertext 𝑐 (see the description of K-PKE.Encrypt). Here, one can think of 𝐮′  as the coefficients 
of the equation and 𝑣′  as the constant term. The decryption key dkPKE  contains the vector of 
secret variables 𝐬. The decryption algorithm can thus use the decryption key to compute the 
true constant term 𝑣 = 𝐬⊺𝐮′  and calculate 𝑣′ − 𝑣. The decryption algorithm ends by decoding 
the plaintext message 𝑚 from 𝑣′ − 𝑣 and outputting 𝑚. 

∈ 𝔹384𝑘 
. 

Algorithm 15 K-PKE.Decrypt(dkPKE, 𝑐) 
Uses the decryption key to decrypt a ciphertext. 
Input: decryption key dkPKE 
Input: ciphertext 𝑐 ∈ 𝔹32(𝑑𝑢𝑘+𝑑𝑣). 
Output: message 𝑚 ∈ 𝔹32 . 
1:  𝑐1  ← 𝑐[0 ∶ 32𝑑𝑢𝑘] 
2:  𝑐2  ← 𝑐[32𝑑𝑢𝑘 ∶ 32(𝑑𝑢𝑘 + 𝑑𝑣)] 
3:  𝐮′  ← Decompress𝑑𝑢
4:  𝑣′  ← Decompress𝑑𝑣
5: 
6:  𝑤 ← 𝑣′ − NTT−1(𝐬⊺̂  ∘ NTT(𝐮′)) 
7:  𝑚 ← ByteEncode1(Compress1(𝑤)) 
8:  return 𝑚 

(ByteDecode𝑑𝑢
(ByteDecode𝑑𝑣
𝐬 ← ByteDecode12(dkPKE 
)

(𝑐1))  ▷ run Decompress𝑑𝑢 
(𝑐2)) 

and ByteDecode𝑑𝑢 

𝑘 times 

▷ run ByteDecode12 
𝑘 times 
▷ run NTT 𝑘 times; run NTT−1  once 
▷ decode plaintext 𝑚 from polynomial 𝑣 

31 

̂
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

6.  Main  Internal  Algorithms 

This section specifies three algorithms:  ML-KEM.KeyGen_internal,  ML-KEM.Encaps_internal, 
and ML-KEM.Decaps_internal. These three algorithms are all deterministic, meaning that their 
output is completely determined by their input.  No randomness is sampled inside of these 
algorithms. These three algorithms will be used to construct ML-KEM in Section 7. The algorithms 
in this section make use of the parameters 𝑛, 𝑞, 𝑘, 𝑑𝑢, and 𝑑𝑣.  The subroutines they invoke 
additionally make use of the parameters 𝜂1  and 𝜂2. While 𝑛 is always 256 and 𝑞 is always 3329, 
the remaining parameters vary among the possible parameter sets (see Section 8). 

The interfaces specified in this section will be used to test ML-KEM implementations through 
the Cryptographic Algorithm Validation Program (CAVP). The key generation function in this 
section may also be used to re-expand a key from a seed (see Section 3.3), including when 
obtaining assurance of private key possession via regeneration. As prescribed in Section 3.3, the 
interfaces specified in this section should not be made available to applications other than for 
testing purposes, and the random seeds (as specified in ML-KEM.KeyGen and ML-KEM.Encaps 
in Section 7) shall be generated by the cryptographic module. 

6.1 

Internal  Key  Generation 

The algorithm ML-KEM.KeyGen_internal (Algorithm 16) accepts two random seeds as input, and 
produces an encapsulation key and a decapsulation key. 

Informal description.  The core subroutine of ML-KEM.KeyGen_internal is the key generation 
algorithm of K-PKE (Algorithm 13). The encapsulation key is simply the encryption key of K-PKE. 
The decapsulation key consists of the decryption key of K-PKE, the encapsulation key, a hash 
of the encapsulation key, and a random 32-byte value. This random value will be used in the 
”implicit rejection” mechanism of the internal decapsulation algorithm (Algorithm 18). 

Algorithm 16 ML-KEM.KeyGen_internal(𝑑, 𝑧) 
Uses randomness to generate an encapsulation key and a corresponding decapsulation key. 
Input: randomness 𝑑 ∈ 𝔹32 . 
Input: randomness 𝑧 ∈ 𝔹32 . 
Output: encapsulation key ek ∈ 𝔹384𝑘+32 . 
Output: decapsulation key dk ∈ 𝔹768𝑘+96 . 
1:  (ekPKE, dkPKE) ← K-PKE.KeyGen(𝑑) 
2:  ek ← ekPKE 
3:  dk ← (dkPKE‖ek‖H(ek)‖𝑧) 
4:  return (ek, dk) 

▷ run key generation for K-PKE 
▷ KEM encaps key is just the PKE encryption key 
▷ KEM decaps key includes PKE decryption key 

6.2 

Internal  Encapsulation 

The algorithm ML-KEM.Encaps_internal (Algorithm 17) accepts an encapsulation key and a ran-
dom byte array as input and outputs a ciphertext and a shared key. 

32 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Algorithm 17 ML-KEM.Encaps_internal(ek, 𝑚) 
Uses the encapsulation key and randomness to generate a key and an associated ciphertext. 
Input: encapsulation key ek ∈ 𝔹384𝑘+32 . 
Input: randomness 𝑚 ∈ 𝔹32 . 
Output: shared secret key 𝐾 ∈ 𝔹32 . 
Output: ciphertext 𝑐 ∈ 𝔹32(𝑑𝑢𝑘+𝑑𝑣). 
1:  (𝐾, 𝑟) ← G(𝑚‖H(ek)) 
2:  𝑐 ← K-PKE.Encrypt(ek, 𝑚, 𝑟) 
3:  return (𝐾, 𝑐) 

▷ derive shared secret key 𝐾 and randomness 𝑟 
▷ encrypt 𝑚 using K-PKE with randomness 𝑟 

Informal description. The core subroutine of ML-KEM.Encaps_internal is the encryption algo-
rithm of K-PKE, which is used to encrypt a random value 𝑚 into a ciphertext 𝑐.  A copy of the 
shared secret key 𝐾 and the randomness used during encryption are derived from 𝑚 and the 
encapsulation key ek via hashing. Specifically, H is applied to ek, and the result is concatenated 
with 𝑚 and then hashed using G. Finally, the algorithm outputs the shared secret key 𝐾 and the 
ciphertext 𝑐. 

6.3 

Internal  Decapsulation 

The algorithm ML-KEM.Decaps_internal (Algorithm 18) accepts a decapsulation key and a cipher-
text as input, does not use any randomness, and outputs a shared secret key. 

Informal description. The algorithm ML-KEM.Decaps_internal begins by parsing out the com-
ponents of the decapsulation key dk of ML-KEM.  These components are an (encryption key, 
decryption key) pair for K-PKE, a hash value ℎ, and a random value 𝑧.  The decryption key of 
K-PKE is then used to decrypt the input ciphertext 𝑐 to get a plaintext 𝑚′ .  The decapsulation 
algorithm then re-encrypts 𝑚′  and computes a candidate shared secret key 𝐾′  in the same 
manner as should have been done in encapsulation.  Specifically, 𝐾′  and the encryption ran-
domness 𝑟′  are computed by hashing 𝑚′  and the encryption key of K-PKE, and a ciphertext 𝑐′  is 
generated by encrypting 𝑚′  using randomness 𝑟′ .  Finally, decapsulation checks whether the 
resulting ciphertext 𝑐′  matches the provided ciphertext 𝑐. If it does not, the algorithm performs 
an “implicit rejection”: the value of 𝐾′ is changed to a hash of 𝑐 together with the random value 
𝑧 stored in the ML-KEM secret key (see the discussion of decapsulation failures in Section  3.2). 
In either case, decapsulation outputs the resulting shared secret key 𝐾′ . 

The “implicit reject” flag computed in step 9 (by comparing 𝑐 and 𝑐′ ) is a secret piece of interme-
diate data. As specified in the requirements in Section 3.3, this flag shall be destroyed prior to 
ML-KEM.Decaps_internal terminating. In particular, returning the value of the flag as an output 
in any form is not permitted. 

33 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Algorithm 18 ML-KEM.Decaps_internal(dk, 𝑐) 
Uses the decapsulation key to produce a shared secret key from a ciphertext. 
Input: decapsulation key dk ∈ 𝔹768𝑘+96 . 
Input: ciphertext 𝑐 ∈ 𝔹32(𝑑𝑢𝑘+𝑑𝑣). 
Output: shared secret key 𝐾 ∈ 𝔹32 . 
1:  dkPKE  ← dk[0 ∶ 384𝑘] 
2:  ekPKE  ← dk[384𝑘 ∶ 768𝑘 + 32] 
3:  ℎ ← dk[768𝑘 + 32 ∶ 768𝑘 + 64] 
4:  𝑧 ← dk[768𝑘 + 64 ∶ 768𝑘 + 96] 
5:  𝑚′  ← K-PKE.Decrypt(dkPKE, 𝑐) 
6:  (𝐾′, 𝑟′) ← G(𝑚′‖ℎ)
7:  𝐾 ← J(𝑧‖𝑐) 
8:  𝑐′  ← K-PKE.Encrypt(ekPKE, 𝑚′, 𝑟′) 
9:  if 𝑐 ≠ 𝑐′  then 
10: 
11:  end if 
12:  return 𝐾′ 

𝐾′ ← 𝐾̄ 

▷ extract (from KEM decaps key) the PKE decryption key 
▷ extract PKE encryption key 
▷ extract hash of PKE encryption key 
▷ extract implicit rejection value 
▷ decrypt ciphertext 

▷ re-encrypt using the derived randomness 𝑟′ 

▷ if ciphertexts do not match, “implicitly reject” 

34 

̄
FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

7.  The  ML-KEM  Key-Encapsulation  Mechanism 

This section describes the three main algorithms of the ML-KEM scheme: 

1.  Key generation (ML-KEM.KeyGen) 

2.  Encapsulation (ML-KEM.Encaps) 

3.  Decapsulation (ML-KEM.Decaps) 

To instantiate ML-KEM, one must select a parameter set. Each parameter set is associated with 
a particular trade-off between security and performance. The three possible parameter sets are 
called ML-KEM-512, ML-KEM-768, and ML-KEM-1024 and are described in detail in Table 2 of 
Section 8. Each parameter set assigns specific numerical values to the individual parameters 𝑛, 
𝑞, 𝑘, 𝜂1, 𝜂2, 𝑑𝑢, and 𝑑𝑣. While 𝑛 is always 256 and 𝑞 is always 3329, the remaining parameters 
vary among the three parameter sets. Implementers shall ensure that the three algorithms of 
ML-KEM listed above are only invoked with a valid parameter set, and that this parameter set is 
selected appropriately for the desired application. Moreover, implementers shall ensure that the 
parameter set used in any particular invocation of ML-KEM.Encaps or ML-KEM.Decaps matches 
the parameter set associated to the provided inputs. 

7.1  ML-KEM  Key  Generation 

The key generation algorithm ML-KEM.KeyGen for ML-KEM (Algorithm 19) accepts no input, 
generates randomness internally, and produces an encapsulation key and a decapsulation key. 
While the encapsulation key can be made public, the decapsulation key shall remain private. 

The seed (𝑑, 𝑧) generated in steps 1 and 2 of ML-KEM.KeyGen can be stored for later expan-
sion using ML-KEM.KeyGen_internal (see Section 3.3).  As the seed can be used to compute 
the decapsulation key, it is sensitive data and shall be treated with the same safeguards as a 
decapsulation key (see SP 800-227 [1]). 

$ 

Algorithm 19 ML-KEM.KeyGen() 
Generates an encapsulation key and a corresponding decapsulation key. 
Output: encapsulation key ek ∈ 𝔹384𝑘+32 . 
Output: decapsulation key dk ∈ 𝔹768𝑘+96 . 
1:  𝑑 ←− 𝔹32 
2:  𝑧 ←− 𝔹32 
3:  if 𝑑 == NULL or 𝑧 == NULL then 
4: 
5:  end if 
6:  (ek, dk) ← ML-KEM.KeyGen_internal(𝑑, 𝑧) 
7:  return (ek, dk) 

return ⊥

$ 

▷ return an error indication if random bit generation failed 

▷ run internal key generation algorithm 

▷ 𝑑 is 32 random bytes (see Section 3.3)
▷ 𝑧 is 32 random bytes (see Section 3.3) 

Secure key establishment depends on the use of key pairs that have been properly generated 
via ML-KEM.KeyGen. If the owner of a KEM key pair did not generate the key pair but instead 
received it from a trusted third party or other source, the owner may optionally perform certain 

35 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

checks on the key pair. While these checks can detect certain corruptions, they do not guarantee 
that the key pair was properly generated. 

Key pair check. To check a candidate key pair1 

(ek, dk), perform the following checks: 

1.  (Seed consistency) If a seed (𝑑, 𝑧) is available, run ML-KEM.KeyGen_internal(𝑑, 𝑧), and 

verify that the output is equal to (ek, dk). 

2.  (Encapsulation key check) Check ek as specified in Section 7.2. 

3.  (Decapsulation key check) Check dk as specified in Section 7.3. 

4.  (Pair-wise consistency) Perform the following steps: 

i.  Generate an array of 32 random bytes by performing 𝑚 ←− 𝔹32 . 

$ 

ii.  Perform (𝐾, 𝑐) ← ML-KEM.Encaps_internal(ek, 𝑚). 
iii.  Perform 𝐾′  ← ML-KEM.Decaps_internal(dk, 𝑐). 
iv.  Reject unless 𝐾 == 𝐾′ . 

It is important to note that this checking process does not guarantee that the key pair is a properly 
produced output of ML-KEM.KeyGen. 

7.2  ML-KEM  Encapsulation 

The encapsulation algorithm ML-KEM.Encaps of ML-KEM (Algorithm 20) accepts an encapsula-
tion key as input, generates randomness internally, and outputs a ciphertext and a shared key. 
This algorithm requires input checking, as specified below. 

Encapsulation key check. To check a candidate encapsulation key ek, perform the following: 

1.  (Type check) If ek is not an array of bytes of length 384𝑘 + 32 for the value of 𝑘 specified 

by the relevant parameter set, then input checking failed. 

2.  (Modulus check) Perform the computation 

test ← ByteEncode12(ByteDecode12(ek[0 ∶ 384𝑘])) 

(7.1) 

(see Section 4.2.1). If test ≠ ek[0 ∶ 384𝑘], then input checking failed. This check ensures 
that the integers encoded in the public key are in the valid range [0, 𝑞 − 1]. 

If both checks pass, then ML-KEM.Encaps can be run with input ek ∶= ek.  It is important to 
note that this checking process does not guarantee that ek is a properly produced output of 
ML-KEM.KeyGen. 

ML-KEM.Encaps shall not be run with an encapsulation key that has not been checked as above. 
However, checking of the encapsulation key need not be performed by the encapsulating party, 

1In discussions of input checking, the “low overline” in the notation indicates that the input has not yet been 
checked (e.g., ek has not yet been checked, while ek has passed the check). 

36 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

nor with every execution of ML-KEM.Encaps. Instead, assurance that these checks have been 
performed can be acquired through other means (see SP 800-227 [1]). 

Algorithm 20 ML-KEM.Encaps(ek) 
Uses the encapsulation key to generate a shared secret key and an associated ciphertext. 
Checked input: encapsulation key ek ∈ 𝔹384𝑘+32 . 
Output: shared secret key 𝐾 ∈ 𝔹32 . 
Output: ciphertext 𝑐 ∈ 𝔹32(𝑑𝑢𝑘+𝑑𝑣). 
1: 
2: 
3: 
4:  end if 
5: 
6: 

(𝐾, 𝑐) ← ML-KEM.Encaps_internal(ek, 𝑚) 
return (𝐾, 𝑐) 

𝑚 ←− 𝔹32 
if 𝑚 == NULL then 

return ⊥ 

$

▷ return an error indication if random bit generation failed 

▷ 𝑚 is 32 random bytes (see Section 3.3) 

▷ run internal encapsulation algorithm 

7.3  ML-KEM  Decapsulation 

The decapsulation algorithm ML-KEM.Decaps of ML-KEM (Algorithm 21) accepts a decapsulation 
key and an ML-KEM ciphertext as input, does not use any randomness, and outputs a shared 
secret. This algorithm requires input checking, as specified below. 

Decapsulation input check. To check a candidate decapsulation key dk and ciphertext 𝑐, perform 
the following checks: 

1.  (Ciphertext type check) If 𝑐 is not a byte array of length 32(𝑑𝑢𝑘 + 𝑑𝑣) for the values of 𝑑𝑢, 

𝑑𝑣, and 𝑘 specified by the relevant parameter set, then input checking has failed. 

2.  (Decapsulation key type check) If dk is not a byte array of length 768𝑘 + 96 for the value of 

𝑘 specified by the relevant parameter set, then input checking has failed. 

3.  (Hash check) Perform the computation 

test ← H(dk[384𝑘 ∶ 768𝑘 + 32])) . 

(7.2) 

If test ≠ dk[768𝑘 + 32 ∶ 768𝑘 + 64], then input checking has failed. 

If all of the above checks pass, then ML-KEM.Decaps can be run with inputs dk ∶= dk and 𝑐 ∶= 𝑐. It 
is important to note that this checking process does not guarantee that dk is a properly produced 
output of ML-KEM.KeyGen, nor that 𝑐 is a properly produced output of ML-KEM.Encaps. 

ML-KEM.Decaps shall not be run with a decapsulation key or a ciphertext unless both have 
been checked.  However, checking of the decapsulation key need not be performed by the 
decapsulating party, nor with every execution of ML-KEM.Decaps. Instead, assurance that this 
check has been performed can be acquired through other means (see SP 800-227 [1]). Ciphertext 
checking shall be performed with every execution of ML-KEM.Decaps. 

37 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Algorithm 21 ML-KEM.Decaps(dk, 𝑐) 
Uses the decapsulation key to produce a shared secret key from a ciphertext. 
Checked input: decapsulation key dk ∈ 𝔹768𝑘+96 . 
Checked input: ciphertext 𝑐 ∈ 𝔹32(𝑑𝑢𝑘+𝑑𝑣). 
Output: shared secret key 𝐾 ∈ 𝔹32 . 
1:  𝐾′  ← ML-KEM.Decaps_internal(dk, 𝑐) 
2:  return 𝐾′ 

▷ run internal decapsulation algorithm 

38 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

8.  Parameter  Sets 

ML-KEM is equipped with three parameter sets, each of the which comprises five individual 
parameters: 𝑘, 𝜂1, 𝜂2, 𝑑𝑢, and 𝑑𝑣.  There are also two constants: 𝑛 = 256 and 𝑞 = 3329.  The 
following is a brief and informal description of the roles played by the variable parameters in the 
algorithms of K-PKE and ML-KEM. See Section 5 for details. 

•  The parameter 𝑘 determines the dimensions of the matrix 𝐀̂ that appears in K-PKE.KeyGen 
and K-PKE.Encrypt. It also determines the dimensions of vectors 𝐬 and 𝐞 in K-PKE.KeyGen 
and the dimensions of vectors 𝐲 and 𝐞1  in K-PKE.Encrypt. 

•  The parameter 𝜂1  is required to specify the distribution for generating the vectors 𝐬 and 𝐞 

in K-PKE.KeyGen and the vector 𝐲 in K-PKE.Encrypt. 

•  The parameter 𝜂2  is required to specify the distribution for generating the vectors 𝐞1  and 

𝑒2  in K-PKE.Encrypt. 

•  The parameters 𝑑𝑢  and 𝑑𝑣  serve as parameters and inputs for the functions Compress, 

Decompress, ByteEncode, and ByteDecode in K-PKE.Encrypt and K-PKE.Decrypt. 

This standard approves the parameter sets given in Table 2. Each parameter set is associated 
with a required security strength for randomness generation (see Section 3.3). The sizes of the 
ML-KEM keys and ciphertexts for each parameter set are summarized in Table 3. 

Table 2. Approved parameter sets for ML-KEM 

𝑞 

𝑛 
ML-KEM-512  256  3329  2  3 
ML-KEM-768  256  3329  3  2 
ML-KEM-1024  256  3329  4  2 

𝑘  𝜂1  𝜂2  𝑑𝑢  𝑑𝑣 
2  10  4 
2  10  4 
2  11  5 

required RBG strength (bits) 
128 
192 
256 

Table 3. Sizes (in bytes) of keys and ciphertexts of ML-KEM 

encapsulation key  decapsulation key  ciphertext  shared secret key 

ML-KEM-512 
ML-KEM-768 
ML-KEM-1024 

800 
1184 
1568 

1632 
2400 
3168 

768 
1088 
1568 

32 
32 
32 

A parameter set name can also be said to denote a (parameter-free) KEM. Specifically, ML-KEM-𝑥 
can be used to denote the parameter-free KEM that results from instantiating the scheme 
ML-KEM with the parameter set ML-KEM-𝑥. 

The three parameter sets included in Table 2 were designed to meet certain security strength 
categories defined by NIST in its original Call for Proposals [4, 22].  These security strength 
categories are explained further in SP 800-57, Part 1 [7]. 

Using this approach, security strength is not described by a single number, such as “128 bits of 
security.” Instead, each ML-KEM parameter set is claimed to be at least as secure as a generic 
39 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

block cipher with a prescribed key size or a generic hash function with a prescribed output 
length. More precisely, it is claimed that the computational resources needed to break ML-KEM 
are greater than or equal to the computational resources needed to break the block cipher or 
hash function when those computational resources are estimated using any realistic model of 
computation. Different models of computation can be more or less realistic and, accordingly, 
lead to more or less accurate estimates of security strength. Some commonly studied models 
are discussed in [23]. 

Concretely, ML-KEM-512 is claimed to be in security category 1, ML-KEM-768 is claimed to be 
in security category 3, and ML-KEM-1024 is claimed to be in security category 5. For additional 
discussion of the security strength of MLWE-based cryptosystems, see [4]. 

Selecting an appropriate parameter set. When initially establishing cryptographic protections 
for data, the strongest possible parameter set should be used. This has a number of advantages, 
including reducing the likelihood of costly transitions to higher-security parameter sets in the 
future.  At the same time, it should be noted that some parameter sets might have adverse 
performance effects for the relevant application (e.g., the algorithm may be unacceptably slow, 
or objects such as keys or ciphertexts may be unacceptably large). 

NIST recommends using ML-KEM-768 as the default parameter set, as it provides a large security 
margin at a reasonable performance cost.  In cases where this is impractical or even higher 
security is required, other parameter sets may be used. 

40 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

References 

[1]  National  Institute  of  Standards  and  Technology  (2024)  Recommendations  for  key-
encapsulation mechanisms, (National Institute of Standards and Technology, Gaithers-
burg,  MD),  NIST  Special  Publication  (SP)  800-227.  [Forthcoming;  will  be  available  at 
https://csrc.nist.gov/publications]. 

[2]  Barker EB, Chen L, Roginsky AL, Vassilev A, Davis R (2018) Recommendation for pair-wise 
key-establishment schemes using discrete logarithm cryptography (National Institute of 
Standards and Technology, Gaithersburg, MD), NIST Special Publication (SP) 800-56A Revi-
sion 3. https://doi.org/10.6028/NIST.SP.800-56Ar3. 

[3]  Barker EB, Chen L, Roginsky AL, Vassilev A, Davis R, Simon S (2019) Recommendation for 
pair-wise key-establishment using integer factorization cryptography (National Institute 
of Standards and Technology, Gaithersburg, MD), NIST Special Publication (SP) 800-56B 
Revision 2. https://doi.org/10.6028/NIST.SP.800-56Br2. 

[4]  Avanzi R, Bos J, Ducas L, Kiltz E, Lepoint T, Lyubashevsky V, Schanck JM, Schwabe P, Seiler G, 
Stehlé D (2020) CRYSTALS-Kyber algorithm specifications and supporting documentation, 
Third-round submission to the NIST’s post-quantum cryptography standardization process. 
Available at https://csrc.nist.gov/Projects/post-quantum-cryptography/post-quantum-cry 
ptography-standardization/round-3-submissions. 

[5]  National Institute of Standards and Technology (2015) Secure hash standard (SHS), (U.S. 
Department of Commerce, Washington, DC), Federal Information Processing Standards 
Publication (FIPS) 180-4. https://doi.org/10.6028/NIST.FIPS.180-4. 

[6]  National Institute of Standards and Technology (2015) SHA-3 standard: Permutation-based 
hash and extendable-output functions, (U.S. Department of Commerce, Washington, DC), 
Federal Information Processing Standards Publication (FIPS) 202. https://doi.org/10.6028/ 
NIST.FIPS.202. 

[7]  Barker EB (2020) Recommendation for key management: Part 1 - General, (National Institute 
of Standards and Technology, Gaithersburg, MD), NIST Special Publication (SP) 800-57 Part 
1, Rev. 5 [or as amended]. https://doi.org/10.6028/NIST.SP.800-57pt1r5. 

[8]  Bos J, Ducas L, Kiltz E, Lepoint T, Lyubashevsky V, Schanck JM, Schwabe P, Seiler G, Stehlé 
D (2018) CRYSTALS-Kyber: A CCA-secure module-lattice-based KEM. 2018 IEEE European 
Symposium on Security and Privacy (EuroS&P), pp 353–367. https://doi.org/10.1109/Euro 
SP.2018.00032. 

[9]  Langlois A, Stehlé D (2015) Worst-case to average-case reductions for module lattices. 
Designs, Codes and Cryptography 75(3):565–599. https://doi.org/10.1007/s10623-014-9 
938-4. 

[10]  Regev O (2005) On lattices, learning with errors, random linear codes, and cryptography. 
Proceedings of the Thirty-Seventh Annual ACM Symposium on Theory of Computing STOC 
’05 (Association for Computing Machinery, New York, NY, USA), pp 84––93. https://doi.org/ 
10.1145/1060590.1060603. 

41 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

[11]  Fujisaki E, Okamoto T (2013) Secure integration of asymmetric and symmetric encryption 
schemes. Journal of Cryptology 26:80–101. https://doi.org/10.1007/s00145-011-9114-1. 

[12]  Hofheinz D, Hövelmanns K, Kiltz E (2017) A modular analysis of the Fujisaki-Okamoto trans-
formation. Theory of Cryptography, eds Kalai Y, Reyzin L (Springer International Publishing, 
Cham), pp 341–371. https://doi.org/10.1007/978-3-319-70500-2_12. 

[13]  Katz J, Lindell Y (2020) Introduction to Modern Cryptography (Chapman & Hall/CRC), 3rd Ed. 

[14]  Almeida JB, Olmos SA, Barbosa M, Barthe G, Dupressoir F, Grégoire B, Laporte V, Léchenet JC, 
Low C, Oliveira T, Pacheco H, Quaresma M, Schwabe P, Strub PY (2024) Formally verifying Ky-
ber episode V: Machine-checked IND-CCA security and correctness of ML-KEM in EasyCrypt, 
Cryptology ePrint Archive, Paper 2024/843. Available at https://eprint.iacr.org/2024/843. 

[15]  Ducas L, Schanck J (2021) Security estimation scripts for Kyber and Dilithium, Github reposi-

tory. Available at https://github.com/pq-crystals/security-estimates. 

[16]  Chen L (2022) Recommendation for key derivation using pseudorandom functions, (National 
Institute of Standards and Technology, Gaithersburg, MD), NIST Special Publication (SP) 
800-108r1-upd1, Includes updates as of February 2, 2024. https://doi.org/10.6028/NIST.SP. 
800-108r1-upd1. 

[17]  Barker EB, Chen L, Davis R (2020) Recommendation for key-derivation methods in key-
establishment schemes (National Institute of Standards and Technology, Gaithersburg, MD), 
NIST Special Publication (SP) 800-56C Revision 2. https://doi.org/10.6028/NIST.SP.800-56C 
r2. 

[18]  Barker EB, Kelsey JM (2015) Recommendation for random number generation using deter-
ministic random bit generators, (National Institute of Standards and Technology, Gaithers-
burg, MD), NIST Special Publication (SP) 800-90A, Rev. 1. https://doi.org/10.6028/NIST.SP. 
800-90Ar1. 

[19]  Sönmez Turan M, Barker EB, Kelsey JM, McKay KA, Baish ML, Boyle M (2018) Recom-
mendation for the entropy sources used for random bit generation, (National Institute 
of Standards and Technology, Gaithersburg, MD), NIST Special Publication (SP) 800-90B. 
https://doi.org/10.6028/NIST.SP.800-90B. 

[20]  Barker E, Kelsey J, McKay K, Roginsky A, Turan MS (2024) Recommendation for random bit 
generator (RBG) constructions, (National Institute of Standards and Technology, Gaithers-
burg, MD), NIST Special Publication (SP) 800-90C 4pd. https://doi.org/10.6028/NIST.SP.80 
0-90C.4pd. 

[21]  Kelsey J, Chang S, Perlner R (2016) SHA-3 Derived Functions: cSHAKE, KMAC, TupleHash 
and ParallelHash, (National Institute of Standards and Technology, Gaithersburg, MD), NIST 
Special Publication (SP) 800-185 [or as amended]. https://doi.org/10.6028/NIST.SP.800-1 
85. 

[22]  National Institute of Standards and Technology (2016) Submission requirements and eval-
uation criteria for the post-quantum cryptography standardization process. Available at 
https://csrc.nist.gov/CSRC/media/Projects/Post-Quantum-Cryptography/documents/call-f 
or-proposals-final-dec-2016.pdf. 

42 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

[23]  Alagic G, Apon D, Cooper D, Dang Q, Dang T, Kelsey J, Lichtinger J, Liu YK, Miller C, Moody 
D, Peralta R, Perlner R, Robinson A, Smith-Tone D (2022) Status report on the third round 
of the NIST post-quantum cryptography standardization process (National Institute of 
Standards and Technology, Gaithersburg, MD), NIST Interagency or Internal Report (IR) 
8413. https://doi.org/10.6028/NIST.IR.8413-upd1. 

[24]  CRYSTALS-Kyber submission team (2023) “Discussion about Kyber’s tweaked FO transform”, 
PQC-Forum Post. Available at https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/W 
FRDl8DqYQ4. 

[25]  CRYSTALS-Kyber submission team (2023) “Kyber decisions, part 2:  FO transform”, PQC-
Forum Post. Available at https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/C0D3W 
1KoINY/m/99kIvydoAwAJ. 

43 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Appendix  A  —  Precomputed  Values  for  the  NTT 

The following 128 numbers are the values of 𝜁BitRev7(𝑖)  mod 𝑞 for 𝑖 ∈ {0, … , 127}. These numbers 
are used in Algorithms 9 and 10. 

{ 1 

1729  2580  3289  2642  630  1897 

848 

1062  1919  193 

797  2786  3260  569 

1746 

296  2447  1339  1476  3046 

56 

2240 

1333 

1426  2094  535  2882  2393  2879  1974 

821 

289 

331  3253  1756  1197  2304  2277 

2055 

650  1977  2513  632  2865 

33 

1320 

1915 

2319  1435  807 

452  1438  2868  1534 

2402 

2647  2617  1481  648  2474  3110  1227 

910 

17 

2761  583  2649  1637  723  2288 

1100 

1409  2662  3281  233 

756  2156  3015 

3050 

1703  1651  2789  1789  1847  952  1461 

2687 

939  2308  2437  2388  733  2337  268 

641 

1584  2298  2037  3220  375  2549  2090 

1645 

1063  319  2773  757  2099  561  2466 

2594 

2804  1092  403  1026  1143  2150  2775 

886 

1722  1212  1874  1029  2110  2935  885  2154 } 

44 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

When implementing Algorithm 11, the values 𝜁2BitRev7(𝑖)+1  mod 𝑞 need to be computed. The 
following array contains these values for 𝑖 ∈ {0, … , 127}: 

{ 17 

-17 

2761 

-2761  583 

-583 

2649 

-2649 

1637 

-1637  723 

-723 

2288 

-2288  1100 

-1100 

1409 

-1409  2662 

-2662  3281 

-3281  233 

-233 

756 

-756 

2156 

-2156  3015 

-3015  3050 

-3050 

1703 

-1703  1651 

-1651  2789 

-2789  1789 

-1789 

1847 

-1847  952 

-952 

1461 

-1461  2687 

-2687 

939 

-939 

2308 

-2308  2437 

-2437  2388 

-2388 

733 

-733 

2337 

-2337  268 

-268 

641 

-641 

1584 

-1584  2298 

-2298  2037 

-2037  3220 

-3220 

375 

-375 

2549 

-2549  2090 

-2090  1645 

-1645 

1063 

-1063  319 

-319 

2773 

-2773  757 

-757 

2099 

-2099  561 

-561 

2466 

-2466  2594 

-2594 

2804 

-2804  1092 

-1092  403 

-403 

1026 

-1026 

1143 

-1143  2150 

-2150  2775 

-2775  886 

-886 

1722 

-1722  1212 

-1212  1874 

-1874  1029 

-1029 

2110 

-2110  2935 

-2935  885 

-885 

2154 

-2154 } 

45 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Appendix  B  —  SampleNTT  Loop  Bounds 

In SampleNTT (Algorithm 7), the algorithm repeatedly generates byte arrays from the XOF to 
create an element of 𝑇𝑞. If a generated byte array value is out of bounds for a coefficient of 𝑇𝑞, 
the algorithm tries again until all 256 coefficients are created. On average, this while loop will 
resolve within a reasonable number of iterations. However, there may be cases in which the 
generated byte arrays are consistently out of bounds and the algorithm may run for a higher 
number of iterations. 

Implementations should not bound this loop, if at all possible.  An incorrect limit will cause 
interoperability errors, and the chances for SampleNTT to iterate longer become exponentially 
rare. If an implementation does bound the number of iterations of SampleNTT, it shall not use a 
limit lower than those presented in Table 4. The calculated probability of SampleNTT exceeding 
the limit is included and calculated under standard assumptions about the output distributions 
of XOFs and hash functions. 

Table 4. While-loop limits and probabilities of occurrence for SampleNTT 

Number of iterations  Probability of reaching limit 

280 

2−261 

If a limit is used and the number of iterations exceeds the limit, then the algorithm shall destroy 
all intermediate results. If a return value or exception is produced, it shall be the same value for 
any execution in which the maximum number of iterations is exceeded. 

46 

FIPS 203 

MODULE-LATTICE-BASED KEY-ENCAPSULATION MECHANISM 

Appendix  C  —  Differences  From  the  CRYSTALS-Kyber  Submission 

This appendix lists the differences between CRYSTALS-KYBER (as described in [4]) and the ML-KEM 
scheme (specified in this document) that result in differing input-output behavior of the main 
algorithms (i.e., KeyGen, Encaps, Decaps). Since a conforming implementation need only match 
the input-output behavior of these three algorithms (see “Implementations” and Section 3.3 
below), the list does not include any of the numerous differences in how the main algorithms actu-
ally produce outputs from inputs (e.g., via different computational steps or different subroutines), 
nor any differences in presentation between this standard and [4]. 

C.1  Differences Between CRYSTALS-Kyber and FIPS 203 Initial Pub-
lic  Draft 

•  In the third-round specification [4], the shared secret key was treated as a variable-length 
value whose length depended on how it would be used in the relevant application. In this 
specification, the length of the shared secret key is fixed to 256 bits. It can be used directly 
in applications as a symmetric key, or symmetric keys can be derived from it, as specified 
in Section 3.3. 

•  The ML-KEM.Encaps and ML-KEM.Decaps algorithms in this specification use a different 
variant of the Fujisaki-Okamoto transform (see [24, 25]) than the third-round specifica-
tion [4]. Specifically, ML-KEM.Encaps no longer includes a hash of the ciphertext in the 
derivation of the shared secret, and ML-KEM.Decaps has been adjusted to match this 
change. 

•  In the third-round specification [4], the initial randomness 𝑚 in the ML-KEM.Encaps algo-
rithm was first hashed before being used. Specifically, between lines 1 and 2 in Algorithm 
20, there was an additional step that performed the operation 𝑚 ← 𝐻(𝑚). The purpose 
of this step was to safeguard against the use of flawed randomness generation processes. 
As this standard requires the use of NIST-approved randomness generation, this step is 
unnecessary and is not performed in ML-KEM. 

•  This specification includes explicit input checking steps that were not part of the third-round 
specification [4]. For example, ML-KEM.Encaps requires that the byte array containing the 
encapsulation key correctly decodes to an array of integers modulo 𝑞 without any modular 
reductions. 

C.2  Changes  From  FIPS  203  Initial  Public  Draft 

The differences between CRYSTALS-KYBER and ML-KEM as described in Appendix C were included 
in the initial public draft (ipd) of FIPS 203, which was posted on August 24, 2023.  Based on 
comments submitted on the draft ML-KEM, domain separation was added to K-PKE.KeyGen to 
prevent the misuse of keys generated to target one security level from being used for a different 
security level when saving a key as a seed. 
Additionally, FIPS 203 ipd had inadvertently swapped the indices of matrix 𝐀̂ in K-PKE.KeyGen and 
K-PKE.Encrypt. This was changed back in the final version of ML-KEM to match CRYSTALS-KYBER. 

47 

