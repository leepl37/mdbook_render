【메시지 규격】
==

* 주제: 모바일 신분증 송신모드 별 메시지 포맷
* 버전: v1.8.0
* 일자: 2022-05-25
* 작성: LG CNS 컨소시엄


**변경 이력**

|  버전  |                                                                         내용                                                                         |    일자     |
| ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| 1.0.0 | 최초작성                                                                                                                                              | 2021-11-05 |
| 1.1.0 | 메시지별 설명 추가                                                                                                                                     | 2021-11-08 |
| 1.2.0 | - M140 PUSH용 전처리 요청 메시지에 항목 추가<br>- M400 VP 제출 URL 내용 추가<br>- profile 샘플 추가<br>- M400 메시지 제출 URL 설명 추가                     | 2021-11-10 |
| 1.3.0 | M400 메시지 symkey 항목 삭제                                                                                                                          | 2021-11-12 |
| 1.4.0 | - 모든 메시지에 version 항목 추가<br>- M140 메시지: pushtype => pushType으로 변경<br>- M400 메시지: encalg 항목 삭제, vp 세부 항목 추가                     | 2021-11-15 |
| 1.5.0 | M200 메시지의 profile 항목을 base64 인코딩에서 JSON 원문으로 수정                                                                                        | 2021-11-22 |
| 1.6.0 | - M200 메시지의 profile, image 항목 값 'none' => 'link'로 변경<br>- handler 경로 수정 (표1 참조)<br>- M140 메시지 삭제<br>- 인터페이스별 M200 전송방법 추가 | 2021-11-24 |
| 1.6.1 | - M200 메시지의 profile, host를 선택항목으로 변경<br>- 2.2.2.3. App2App 전송방법 설명 추가<br>- M310 응답 메시지를 hexstring에서 string으로 변경            | 2021-11-26 |
| 1.6.2 | M200 메시지의 profile 항목을 string에서 base64 인코딩으로 수정                                                                                          | 2021-12-08 |
| 1.6.3 | M400 메시지의 present 항목 삭제하고 vp 항목에 하위항목으로 presentType, authType 추가                                                                     | 2021-12-17 |
| 1.7.0 | 오류 메시지 및 오류 코드 추가 등<br>상세 내용은 Release Notes 참조                                                                                       | 2021-12-29 |
| 1.7.1 | 2.2.2.2. PUSH 오류 메시지 수정                                                                                                                        | 2022-01-12 |
| 1.7.2 | - PUSH 요청 메시지 생성 방법 수정<br>- Base64 인코딩 가이드 수정                                                                                         | 2022-01-26 |
| 1.8.0 | - PUSH 요청 메시지 Base64 인코딩 가이드 수정<br>- BLE 인터페이스 전송방법 추가                                                                            | 2022-05-25 |

[표1. Default handler 경로]

|     종류     |   Handler    |              예시               |
| ------------ | ------------ | ------------------------------ |
| profile 조회 | /mip/profile | http://example.com/mip/profile |
| image 조회   | /mip/image   | http://example.com/mip/image   |
| VP 제출      | /mip/vp      | http://example.com/mip/vp      |
| 오류 제출     | /mip/error   | http://example.com/mip/error   |




<div style="page-break-after: always;"></div>

# 1. 개요

## 1.1. 용어

* mDL
    * Mobile Driver License의 약어
    * 모바일 운전면허증
* 모바일 신분증
    * 모바일로 발급한 국가 신분증
    * 현재는 모바일 운전면허증(mDL)만 있음
* 신분증앱
    * "모바일 신분증 앱"의 줄임말
* DID (Decentralized ID)
    * 탈중앙화된 신원
* DID Document (DID 문서)
    * DID의 요소로서 블록체인에 등록되어 누구나 조회 가능한 문서
    * DID 소유자의 id (예 - did:kr:mobileid:1234567890) 및 공개키 등이 저장됨
    * W3C의 Decentralized Identifier v.1.0을 준수
* Holder
    * Issuer가 발급한 VC를 소유하는 주체
* Issuer
    * VC를 발급하는 주체
* VC (Verifiable Credential)
    * Issuer(발급자)가 holder의 요청에 의해 holder의 개인정보를 증명 가능한 형태로 발급한 문서
    * W3C의 Verifiable Credential Data Model v.1.0을 준수
* VP (Verifiable Presentation)
    * Holder가 서비스를 제공받거나 기타 용도로 VC를 verifier에게 제출하기 위해 작성하고 서명한 문서
    * 여러 발급자의 여러 VC를 하나의 VP에 담을 수도 있음
* Verifier
    * Holder가 제출한 VP를 검증하는 검증자
    * 일반적으로 서비스를 제공하는 SP(Service Provider)가 verifier 역할을 수행함


## 1.2. 인터페이스 및 전송모드

### 1.2.1. 인터페이스

|   인터페이스   |                      설명                      |          비고           |
| ------------ | --------------------------------------------- | ----------------------- |
| **QR-MPM**   | 응대장치가 QR을 출력하고 신분증앱이 스캔          | Merchant Presented Mode |
| **QR-CPM**   | 신분증앱이 QR을 출력하고 응대장치가 스캔          | Customer Presented Mode |
| **App2App**  | 신분증앱과 서비스앱이 URL scheme으로 직접 통신    |                         |
| **PUSH**     | 조폐공사 PUSH 서버를 통해 신분증앱에 VP 제출 요청 |                         |
| **BLE + QR** | QR코드를 통해 BLE 연결을 수행하고 이후 BLE 통신   | CPM, MPM 모두 지원       |


### 1.2.2. 송신모드

|     모드      |                                  설명                                   |    비고     |
| ------------ | ---------------------------------------------------------------------- | ---------- |
| **direct**   | verifier서버에 VP 직접 제출                                              |            |
| **indirect** | **App2App**: 서비스앱으로 VP 제출<br>**BLE + QR**: BLE 응대장치로 VP 제출 |            |
| **proxy**    | verifier서버와의 통신 환경이 불가하여 중계서버를 이용하고자 하는 경우        | 웹소켓 연결 |


## 1.3. 인코딩 / 디코딩

### 1.3.1. Base64

"Base 64 Encoding with URL and Filename Safe Alphabet"을 사용하며 아래 링크에서 규격을 참조한다.
* https://tools.ietf.org/html/rfc4648#page-7

**Node.js**

```js
const urlsafe_base64 = require('urlsafe-base64');
const { Buffer } = require("buffer");

function base64Encode(str) {
    return urlsafe_base64.encode(Buffer.from(str));
}

function base64Decode(base64) {
    return urlsafe_base64.decode(base64).toString();
}

var base64 = base64Encode('Test string');
console.log(base64);
console.log(base64Decode(base64));
```

**Java 8**

Java8 이전에는 Apache commons의 Base64를 써야 URLSafeBase64 인코딩이 가능했는데, Java8에 포함된 java.util.Base64는 자체적으로 URLSafeBase64를 지원한다.

```java
import java.util.Base64;
import static java.nio.charset.StandardCharsets.UTF_8;

// 인코딩
String rawCookieValue = "(DMPD)b80f9ed8-4e66-41a4-ac60-b46ea5586cf0";
byte[] urlSafeBase64Encoded = Base64.getUrlEncoder().withoutPadding().encode(rawCookieValue.getBytes(UTF_8));
String encodedCookieValue = new String(urlSafeBase64Encoded, UTF_8);
System.out.println(encodedCookieValue);

String encodedFromNodeJs = "KERNUEQpYjgwZjllZDgtNGU2Ni00MWE0LWFjNjAtYjQ2ZWE1NTg2Y2Yw";
System.out.println(encodedFromNodeJs);
System.out.println("nodejs의 urlsafe-base64 인코딩값 == java8의 urlsafe-base64 인코딩값 : " +
        encodedCookieValue.equals(encodedFromNodeJs));

// 디코딩
byte[] decoded = Base64.getUrlDecoder().decode(encodedCookieValue.getBytes(UTF_8));
String urlSafeBase64Decoded = new String(decoded, UTF_8);

System.out.println("++++++++++++");
System.out.println(rawCookieValue);
System.out.println(urlSafeBase64Decoded);
System.out.println("인코딩 전 쿠키값 == 디코딩 후 쿠키값 : " + rawCookieValue.equals(urlSafeBase64Decoded));
```

<div style="page-break-after: always;"></div>


# 2. 메시지 규격

전송모드의 단계별 송수신 메시지의 규격을 정의한다. 본 문서의 모든 메시지는 JSON 형태로 되어 있으며 전송 시 base64 형태로 인코딩한다.

모든 메시지는 아래의 항목을 필수로 포함한다.
* type(M): 'mip'로 고정 (mip = mobile id platform)
* version(M): '1.0.0'으로 고정하고 메시지규격 변경시 버전관리를 통해 하위호환을 보장
* cmd(M): 메시지 고유번호
* trxcode(M): verifier와 신분증앱간 트랜잭션 관리용 코드

※ (M) 표시는 필수항목, (O) 표시는 선택항목

메시지는 크게 다음의 네 가지로 분류한다.

|        단계         |                                         설명                                          |
| ------------------ | ------------------------------------------------------------------------------------ |
| VP 전처리 단계(100) | QR-CPM, QR기반 BLE 등 신분증앱에 직접적으로 VP 요청이 불가한 경우 전처리 요청 메시지를 전송 |
| VP 요청 단계(200)   | VP 제출을 요청하는 단계<br>Profile, BI 이미지 정보 포함 여부는 옵션                       |
| VP 준비 단계(300)   | VP 요청 메시지를 수신 후 필요한 경우 Profile이나 BI 이미지를 수신하는 단계                 |
| VP 제출 단계(400)   | 신분증앱이 verifier에게 VP를 제출하는 단계                                              |
| --- 기타 ---        |                                                                                      |
| 기타 (900)          | 에러 등 단계에 관계없이 전달하는 메시지                                                  |

<span style="color:red">중계서버와의 프로파일, VP 등에 대한 송수신 메시지 규격은 중계서버용 API 매뉴얼을 참고</span>

## 2.1. 100 - VP 전처리 단계

### 2.1.1. [M120] QR-CPM용 전처리 요청 메시지

신분증앱이 중계서버에 거래시작을 요청하고 거래코드를 수신한다. 신분증앱은 M120 메시지를 생성하여 base64 인코딩한 값을 QR로 표시한다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"120",
  "trxcode":"20211102162732145432176790",
  "mode":"proxy",
  "host":"wss://proxy.com:9090"
}
```

[항목]
* trxcode(M)
    * verifier와 신분증앱간 트랜잭션 관리를 위해 중계서버가 생성한 코드로 유일값을 보장해야 함
* mode(M)
    * 'proxy': QR-CPM은 중계서버 모드만 가능
* host(M)
    * Profile 및 VP를 송수신할 중계서버의 호스트명
    * 호스트명 이후의 하위경로는 다른 메시지의 HTTP 헤더에 명시되어 있음

**Base64**

```
ewogICJ0eXBlIjoibWlwIiwKICAidmVyc2lvbiI6IjEuMC4wIiwKICAiY21kIjoiMTIwIiwKICAidHJ4Y29kZSI6IjIwMjExMTAyMTYyNzMyMTQ1NDMyMTc2NzkwIiwKICAibW9kZSI6InByb3h5IiwKICAiaG9zdCI6IndzczovL3Byb3h5LmNvbTo5MDkwIiwKfQ
```

**QR**

![M120 메시지 QR 예시](vx_images/555652209211264.png)

### 2.1.2. [M150] QR기반 BLE용 전처리 요청 메시지

BLE 연결정보를 QR로 교환하는 절차이다. BLE 연결을 대기하는 쪽에서 QR을 생성한다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"150",
  "trxcode":"20211102162732145432176790",
  "bleid":"550e8400-e29b-41d4-a716-446655440000",
  "blepwd":"1234567890"
}
```

[항목]
* trxcode(M)
    * verifier와 신분증앱간 트랜잭션 관리를 위해 BLE advertiser 측이 생성한 코드로 유일값을 보장해야 함
    * QR-CPM의 경우 신분증앱이, QR-MPM인 경우 verifier 측이 생성함
* bleid(M)
    * BLE 연결을 위한 장치 ID
    * 1회용 ID로서 사용후 폐기하고 다른 값을 생성해야 함
* blepwd(M)
    * BLE 연결을 위한 비밀번호
    * 1회용 비밀번호로서 사용후 폐기하고 다른 값을 생성해야 함


## 2.2. 200 - VP 요청 단계

### 2.2.1. [M200] VP 요청 메시지

Verifier가 신분증앱에 VP 제출을 요청하는 메시지이다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"200",
  "trxcode":"20211102162732145432176790",
  "mode":"direct",  -- direct/indriect/proxy
  "profile":"base64 인코딩된 profile",
  "image":"link",
  "ci":false,
  "host":"http://example.com"
}
```

[항목]
* mode(M)
    * 'direct': 신분증앱이 VP를 verifier 서버에 직접 전송
    * 'indirect': 신분증앱으로부터 App2App, BLE 등 간접 인터랙션을 통해 VP를 verifier 서버에 전송
    * 'proxy': verifier 서버와의 직접 통신이 불가하여 중계서버를 이용하는 경우
* profile(O)
    * profile string을 base64 인코딩한 값
    * 항목이 누락된 경우 본 메시지에 profile 항목 미포함. *host*/mip/profile 경로로 profile 요청하여야 함
* image(O)
    * 'link': 본 메시지에 이미지 항목 미포함. *host*/mip/image 경로로 이미지 요청하여야 함
    * 기타: BI 이미지 수신 URL (ex: http://example.com/image/1.jpg)
    * image 항목이 누락되거나 값이 `null`인 경우 이미지를 제공하지 않음
* ci(O)
    * true: VP에 CI를 포함
    * false/null: VP에 CI를 포함하지 않음
* host(O)
    * profile, image 항목을 모두 포함하는 경우 생략 가능
    * 신분증앱에서 profile을 다운로드할 서버의 호스트명
    * 호스트명 이후의 하위경로는 다른 메시지의 HTTP 헤더에 명시되어 있음

### 2.2.2. 인터페이스 별 M200 전송방법

사용하는 인터페이스에 따라 각각 정의된 방식으로 M200 메시지를 신분증앱에 전달한다.

#### 2.2.2.1. QR-MPM

M200 메시지를 base64 인코딩후 QR로 표시한다.

**Base64**

```
ewogICJ0eXBlIjoibWlwIiwKICAidmVyc2lvbiI6IjEuMC4wIiwKICAiY21kIjoiMjAwIiwKICAidHJ4Y29kZSI6IjIwMjExMTAyMTYyNzMyMTQ1NDMyMTc2NzkwIiwKICAibW9kZSI6ImRpcmVjdCIsCiAgInByb2ZpbGUiOiJsaW5rIiwKICAiaW1hZ2UiOiJsaW5rIiwKICAiaG9zdCI6ImV4YW1wbGUuY29tIgp9
```

**QR**

![M200 메시지 QR 예시](vx_images/416514213237624.png)

**주의사항**

<span style="color: red">
QR-MPM 적용 시 M200에 profile은 포함하지 않아야 않아야 한다. profile을 포함하면 데이터 용량이 커져 신분증앱이 QR코드를 제대로 인식하지 못하는 상황이 자주 발생한다.
</span>

#### 2.2.2.2. PUSH

서비스 제공자의 웹 페이지에서 사용자의 전화번호를 입력받아 VP 제출을 요청하는 경우 VP 요청 메시지(M200)는 PUSH 메시지에 담아서 보내야 한다. verifier는 전화번호와 VP 요청 메시지를 아래의 `PUSH 요청 메시지`에 담아 한국조페공사 PUSH 서버에 전송한다.

**PUSH 요청 메시지**

```json
{
  "mscode":"************",
  "pushType":"MIP-USP-001",
  "name":"홍길동",
  "birth":"20010801",
  "telno":"01012345678",
  "data":"ew0KICAidHlwZSI6ICJtaXAiLA0KICAiY21kIjogIjIwMCIsDQogICJ2ZXJzaW9uIjogIjEuMC4wIiwNCiAgInRyeGNvZGUiOiAiMjAyMjAxMjQxODEwNDQ5ODNBQ0MwNTNEMSIsDQogICJtb2RlIjogImRpcmVjdCIsDQogICJpbWFnZSI6ICJsaW5rIiwNCiAgImNpIjogdHJ1ZSwNCiAgImhvc3QiOiAiaHR0cDovL3d3dy55b3Vyc2VydmljZS5jb20iDQp9"
}
```

[항목]
* mscode(M)
    * 검증자 등록시 한국조폐공사에서 발급하는 코드
* pushType(M)
    * PUSH 메시지 타입
    * 'MIP-USP-001'로 고정
* name(O)
    * PUSH 메시지 수신자 성명
* birth(O)
    * PUSH 메시지 수신자 생년월일(YYYYMMDD)
* telno(M)
    * PUSH 메시지를 수신할 전화번호
* data(M)
    * VP 요청 메시지(M200)를 base64 인코딩한 값
    * PUSH 메시지 payload 사이즈 제한으로 아래 사항을 권고함
        * M200.host: (필수) SP서버의 주소
        * M200.profile: 미포함
        * M200.image: "link" 또는 미포함

[PUSH용 M200 예시]

```json
{
  "type": "mip",
  "cmd": "200",
  "version": "1.0.0",
  "trxcode": "20220124181044983ACC053D1",
  "mode": "direct",
  "image": "link",
  "ci": true,
  "host": "http://www.yourservice.com"
}

-- profile 항목이 없음에 유의 => 신분증앱은 http://www.yourservice.com/mip/profile 주소로 M310 메시지 송신하여 profile 획득함
```


**HTTP Request**
PUSH 서버에 전송 시는 해당 메시지를 base64 인코딩해서 전달한다.

* 조폐공사 PUSH 서버 URL: `https://psh.mobileid.go.kr:8443/api/sendVPAPI.do`

```http
POST /api/sendVPAPI.do HTTP/1.1
Host:psh.mobileid.go.kr:8443
Content-type:application/json; charset=UTF-8

{
  "data":"base64 인코딩된 PUSH 요청 메시지"   <== base64 padding 적용 필수
}
```

**HTTP Response (정상)**
PUSH 서버는 상기 요청에 대해 정상인 경우 아래와 같이 응답한다.

```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "key":1,
  "result":true,
  "resultMsg":"SUCCESS",
  "errcode":0,
  "errmsg":"success"
}
```

**HTTP Response (오류)**
오류 발생 시 아래와 같이 응답한다.

```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "key":0,
  "result":false,
  "resultMsg":"NOT_ENOUGH_DATA",
  "errcode":10304,
  "errmsg":"data is missing"
}

```

[가능한 오류 목록]

| 오류코드<br>(10진수) |         이름         |          설명           |
| ------------------- | ------------------- | ---------------------- |
| 10301               | mscode is missing   | 미등록 연계시스템        |
| 10302               | pushType is missing | pushType이 작성되지 않음 |
| 10303               | telno is missing    | telno가 작성되지 않음    |
| 10304               | data is missing     | data가 작성되지 않음     |
| 10305               | receive is missing  | 조회된 수신자 없음       |
| 10306               | insert data fail    | 메시지 정보 INSERT 실패  |
| 10399               | unknown error       | 알 수 없는 오류          |


#### 2.2.2.3. App2App

별도의 App2App 연동 가이드 문서를 참조한다.

#### 2.2.2.4. BLE

신분증앱과 응대장치가 물리적으로 가까운 거리에 있고, BLE로 정보를 송수신하는 경우에 대하여 설명한다. 모드는 아래 두 가지가 있다.

|            모드             |                  설명                  |                            장치의 온라인 여부                            |
| -------------------------- | ------------------------------------- | ---------------------------------------------------------------------- |
| BLE + QR-MPM indirect mode | 응대장치가 QR을 표출하고 BLE 연결을 대기 | - 신분증앱: 온라인<br>- 응대장치: 온라인                                  |
| BLE + QR-CPM indirect mode | 신분증앱이 QR을 표출하고 BLE 연결을 대기 | - 신분증앱: <span style="color:red">오프라인</span><br>- 응대장치: 온라인 |

대부분의 상황에서는 신분증앱이 인터넷에 연결된 상태이므로 `BLE + QR-MPM` 방식의 사용을 권장한다.

##### 2.2.2.4.1. 공통
[메시지]

절차 설명에 나타나는 `password`, `profile` 등의 메시지는 **6.1. BLE 인터페이스 메시지**를 참조한다. 중계서버 메시지와 형식은 유사하나 같은 것이 아님을 유의하여야 한다. 이러한 이유로 <u>BLE + QR 모드에서는 M200 메시지는 사용되지 않는다</u>.

[메시지 전송]

각 메시지 전송 시에는 메시지의 끝을 알리기 위한 고유 문자열 "/EOM/"을 첨부한다. 아래는 그 예시이다.

|       신분증앱        |            응대장치            |
| -------------------- | ----------------------------- |
|                      | ← {"msg":"password",...}/EOM/ |
| {"msg":"ack"}/EOM/ → |                               |


[가능한 오류 목록]

오류 발생 시 `error` 메시지에 오류정보를 담아 전송한다.

| 오류코드<br>(10진수) |        이름         |        설명        |         비고          |
| ------------------- | ------------------ | ------------------ | -------------------- |
| 10501               | incorrect password | BLE 비밀번호 불일치 | 응대장치에서 오류 발생 |
| 20501               | incorrect password | BLE 비밀번호 불일치 | 신분증앱에서 오류 발생 |

##### 2.2.2.4.2. BLE + QR-MPM indirect mode

[절차 요약]
1. 응대장치가 `bleid`, `blepw` 생성하여 QR 표출하고 연결 대기
    * bleid: UUID 형식으로 매번 다른 값을 생성
    * blepd: 10자리의 hexa-decimal 스트링
2. 신분증앱이 QR을 스캔하여 정보획득
3. 신분증앱이 `bleid`로 장치 스캔하여 연결
4. BLE 인터페이스 메시지 형식에 맞게 메시지 송수신
5. 거래 종료

[절차 상세]
```plantuml
@startuml BLE + QR-MPM
'===============================================================================
' 설정
'===============================================================================

'=== Header and Footer ===
'header 모바일 신분증
'footer
'  	Page %page% of %lastpage% at %date("yyyy-MM-dd hh:mm:ss")
'end footer

'=== 제목 ===
title
	<size:25>BLE + QR-MPM indirect mode</size>
end title

'=== 옵션 ===
' 자동채번: ex) "<b>000." => 앞에 0을 채워 3자리 숫자로 표시
autonumber "<b>(#)</b>"
' 페이지 하단 participant box 숨기기
'hide footbox

'=== Panticipants ===
participant "신분증앱" as APP <<Mobile>>
participant "응대장치" as RD <<Device>>


'===============================================================================
' 상수
'===============================================================================


'===============================================================================
' 내용
'===============================================================================

== 선처리 ==
RD -> RD: **M150** 메시지 생성
note right
**-- M150**
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"150",
  "trxcode":"20211102162732145432176790",
  "bleid":"550e8400-e29b-41d4-a716-446655440000",
  "blepwd":"1234567890"
}
end note
RD -> RD: QR 표출
RD -->> APP: QR스캔

group BLE 연결 초기화
    APP -> APP: M150.bleid 로 기기검색
    APP -->> RD: BLE 연결
end

== VP 제출 및 검증 ==
group BLE 인터페이스
    APP -> RD: **password** 메시지 전송
    RD -> RD: 연결 비밀번호(password.blepw) 확인
    RD -> RD: profile 생성
    note right: 필요시 image, ci 항목 설정 가능
    RD -> APP: **profile** 메시지 전송
    APP -> APP: VP 생성
    APP -> RD: **vp** 메시지 전송
    RD -> RD: VP 검증
    RD -> APP: **finish** 메시지 전송
end

== 거래 종료 ==
APP <<-->> RD: BLE 연결 해제

@enduml
```

##### 2.2.2.4.3. BLE + QR-CPM indirect mode
[절차 요약]
1. 신분증앱이 `bleid`, `blepw` 생성하여 QR 표출하고 연결 대기
    * bleid: UUID 형식으로 매번 다른 값을 생성
    * blepd: 10자리의 hexa-decimal 스트링
2. 응대장치가 QR을 스캔하여 정보획득
3. 응대장치가 `bleid`로 장치 스캔하여 연결
4. BLE 인터페이스 메시지 형식에 맞게 메시지 송수신
5. 거래 종료

[절차 상세]
```plantuml
@startuml BLE + QR-MPM
'===============================================================================
' 설정
'===============================================================================

'=== Header and Footer ===
'header 모바일 신분증
'footer
'  	Page %page% of %lastpage% at %date("yyyy-MM-dd hh:mm:ss")
'end footer

'=== 제목 ===
title
	<size:25>BLE + QR-CPM indirect mode</size>
end title

'=== 옵션 ===
' 자동채번: ex) "<b>000." => 앞에 0을 채워 3자리 숫자로 표시
autonumber "<b>(#)</b>"
' 페이지 하단 participant box 숨기기
'hide footbox

'=== Panticipants ===
participant "신분증앱" as APP <<Mobile>>
participant "응대장치" as RD <<Device>>


'===============================================================================
' 상수
'===============================================================================


'===============================================================================
' 내용
'===============================================================================

== 선처리 ==
APP -> APP: **M150** 메시지 생성
note right
**-- M150**
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"150",
  "trxcode":"20211102162732145432176790",
  "bleid":"550e8400-e29b-41d4-a716-446655440000",
  "blepwd":"1234567890"
}
end note
APP -> APP: QR 표출
APP -->> RD: QR스캔

group BLE 연결 초기화
    RD -> RD: M150.bleid 로 기기검색
    APP <<-- RD: BLE 연결
end

== VP 제출 및 검증 ==
group BLE 인터페이스
    APP <- RD: **password** 메시지 전송
    APP -> APP: 연결 비밀번호(password.blepw) 확인
    APP -> RD: **ack** 메시지 전송
    RD -> RD: profile 생성
    note right: 필요시 image, ci 항목 설정 가능
    RD -> APP: **profile** 메시지 전송
    APP -> APP: VP 생성
    APP -> RD: **vp** 메시지 전송
    RD -> RD: VP 검증
    RD -> APP: **finish** 메시지 전송
end

== 거래 종료 ==
APP <<-->> RD: BLE 연결 해제

@enduml
```


#### 2.2.2.5. NFC

<span style="color:red">정의 필요</span>



## 2.3. 300 - VP 준비 단계

### 2.3.1. [M310] Profile 요청 메시지

신분증앱은 VP 요청 메시지(M200)를 참조해서 profile URL로부터 프로파일을 다운로드하기 위해 아래와 같이 메시지를 구성한다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"310",
  "request":"profile",
  "trxcode":"20211102162732145432176790"
}
```

[항목]
* request(M)
    * 'profile' 고정

**HTTP Request**
verifier에 요청 시는 해당 메시지를 base64 인코딩해서 전달한다.
```http
POST /mip/profile HTTP/1.1
Host:http://example.com
Content-type:application/json; charset=UTF-8

{
  "data":"base64 인코딩된 프로파일 요청메시지"
}
```

**HTTP Response (정상)**
검증자는 상기 요청에 대해 아래와 같이 응답 메시지를 구성한다.

```json
{
  "trxcode":"20211102162732145432176790",
  "profile":"base64 인코딩된 profile"
}
```

http 응답코드와 함께 base64 인코딩된 메시지를 응답한다.
```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":true,
  "data":"base64 인코딩된 프로파일 응답메시지"
}
```

**HTTP Response (오류)**
오류 발생 시 아래와 같이 오류 메시지를 구성한다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"900",
  "trxcode":"20211102162732145432176790",
  "errcode":"10002",
  "errmsg":"missing mandatory item: request"
}
```

http 응답코드와 함께 base64 인코딩된 오류 메시지를 응답한다.
```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":false,
  "data":"base64 인코딩된 오류 메시지"
}
```

[가능한 오류 목록]

| 오류코드<br>(10진수) |             이름             |                   설명                    |
| ------------------- | --------------------------- | ----------------------------------------- |
| 10001               | unexpected message format   | JSON 형식의 메시지가 아님                   |
| 10002               | missing mandatory item      | type, version, cmd, request, trxcode 누락 |
| **--- 데이터 ---**  |                             |                                           |
| 10100               | invalid data                | request != 'profile'                      |
| 10101               | invalid type                | type != 'mip'                             |
| 10102               | unsupported message version | 지원하지 않는 메시지 버전                   |
| 10103               | invalid cmd                 | cmd != '310'                              |
| 10104               | trxcode not found           | 존재하지 않는 거래코드                      |
| **--- 절차 ---**    |                             |                                           |
| 10202               | timeout error               | 유효시간 초과 오류                          |
| **--- 기타 ---**    |                             |                                           |
| 99999               | unknown error               | 알 수 없는 오류                            |


**샘플 profile (일반 VP 제출용. presentType=1)**
데이터는 실제와 무관하므로 JSON 형태의 항목만 참조한다.

```json
{
    "encoding": "UTF-8",
    "id": "did:omn:2DsCWzaAnhXeRNDkKWJdE2oypRRB",
    "language": "KR",
    "profile": {
        "authType": [
            "pin"
        ],
        "callBackUrl": "http://example.com/mip/vp",
        "encryptType": 2,
        "filter": {
            "allowIssuerList": [
            ],
            "requiredAssertionList": [
                {
                    "id": "driverlicen",
                    "name": "운전면허증",
                    "type": "driverlicen"
                },
                {
                    "id": "stk",
                    "name": "stk",
                    "type": "stk"
                }
            ],
            "requiredPrivacyList": [
                "address"
            ]
        },
        "keyType": 2,
        "name": "Raonsecure",
        "nonce": "2704b5e4eb5de74c3ff7956c53ae1dc9596a89545fe581755bee82995f7a83a2",
        "presentType": 1,
        "publicKey": "2TuPVgMCHJy5atawrsADEzjP7MCVbyyCA89UW6Wvjp9HrAjbJ1SzHsuJBqBShz1UoKVXLcKgoa4ACC5S579PUQpxFNPfVC3sRcMmeF7N5gPshhr2D7THUxsN6fo8U5Efjp24p8RUiyDS7QrV1W2DTSgFtPtfq2M8FRXFaPDJ2LDDuqDbAdaKdqkgobRBbQeW9B96WT9EGamGA8JYUSfPyw7V2c9gKcMGVisXEEqJFCM4uUpBS7yDXquKyT2EFfoxCRKNTT64FhP3rkLGsSizZhrBYxyuTdcVbnrpVoAgHNPGjTUzcYJ3CAtNGDPggUyeUwZ3Lq1tA9AbUCmHEmBU9dSFeDsxBF3kzTSLW1a3BPR4knK6ZGuUagzL8tYGQ1daoc2yh2mPuCDVjarYHE",
        "spCode": "omni.sp2",
        "svcCode": "zkph2",
        "type": "VERIFY"
    },
    "proof": {
        "created": "2021-11-08T15:52:45",
        "creator": "did:omn:2DsCWzaAnhXeRNDkKWJdE2oypRRB#zkph2",
        "nonce": "7a5ab53fbc208576783248c872a7662c9304bf4dcb331d892c12c9f56b3d1d0f",
        "signatureValue": "3rxczYCHDEpjS4CUWiZwZDt7fznCYcW5Y5mbFgcwWJE2utEbFqDkWCSZPSemoLtpvz1FTvyS2QzEXy9rRZ6Njxifn",
        "type": "Secp256k1VerificationKey2018"
    },
    "timezone": "UTC+9",
    "version": "3.0"
}
```


### 2.3.2. [M320] Image 요청 메시지

신분증앱은 VP 요청 메시지(M200)를 참조해서 이미지 URL로부터 이미지를 다운로드하기 위해 아래와 같이 이미지 요청 메시지를 구성한다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"320",
  "request":"image",
  "trxcode":"20211102162732145432176790"
}
```

[항목]
* request(M)
    * 'image'로 고정

**HTTP Request**
verifier에게 요청시는 해당 메시지를 base64 인코딩해서 전달한다.
```http
POST /mip/image HTTP/1.1
Host:http://example.com
Content-type:application/json; charset=UTF-8

{
  "data":"base64 인코딩된 이미지 요청메시지"
}
```

**HTTP Response (정상)**
http 응답코드와 함께 base64 인코딩된 메시지를 응답한다.
```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":true,
  "data":"base64 인코딩된 이미지 응답메시지"
}
```

**HTTP Response (오류)**
오류 발생 시 아래와 같이 오류 메시지를 구성한다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"900",
  "trxcode":"20211102162732145432176790",
  "errcode":10002,
  "errmsg":"missing mandatory item: request"
}
```

http 응답코드와 함께 base64 인코딩된 오류 메시지를 응답한다.
```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":false,
  "data":"base64 인코딩된 오류 메시지"
}
```

[가능한 오류 목록]

| 오류코드<br>(10진수) |             이름             |                   설명                    |
| ------------------- | --------------------------- | ----------------------------------------- |
| 10001               | unexpected message format   | JSON 형식의 메시지가 아님                   |
| 10002               | missing mandatory item      | type, version, cmd, request, trxcode 누락 |
| **--- 데이터 ---**  |                             |                                           |
| 10100               | invalid data                | request != 'image'                        |
| 10101               | invalid type                | type != 'mip'                             |
| 10102               | unsupported message version | 지원하지 않는 메시지 버전                   |
| 10103               | invalid cmd                 | cmd != '320'                              |
| 10104               | trxcode not found           | 존재하지 않는 거래코드                      |
| **--- 절차 ---**    |                             |                                           |
| 10202               | timeout error               | 유효시간 초과 오류                          |
| **--- 기타 ---**    |                             |                                           |
| 99999               | unknown error               | 알 수 없는 오류                            |


## 2.4. 400 - VP 제출 단계


### 2.4.1. [M400] VP 제출 메시지

신분증앱은 프로파일을 참조해서 아래와 같이 VP 제출 메시지를 생성한다.

M400 메시지 제출 URL
* 1순위: 수신한 profile의 `profile.callBackUrl`에 지정한 URL
* 2순위: M200 메시지의 `host`에 지정된 호스트의 /mip/vp 경로 (*host*/mip/vp)

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"400",
  "request":"presentation",
  "trxcode":"20211102162732145432176790",
  "vp":{...}
}
```

[항목]
* request(M)
    * 'presentation'으로 고정
* trxcode(M)
    * 전처리 요청 메시지(M1XX) 또는 VP 요청 메시지(M200) 내에 지정된 trxcode
* vp(M)
    * presentType(M): 제출타입으로 profile 내의 `presentType`을 그대로 설정
        * 0: DID Auth
        * 1: DID VP
        * 2: ZKP(영지식증명) VP
    * encryptType(M): VP 암호화 여부 및 암호화 알고리즘
        * 0: 암호화 하지 않음
        * 1: AES-128
        * **2: AES-256** (mDL은 VP를 AES-256으로 암호화하며, 암호화에 쓰인 키를 서버의 공개키로 암호화 함)
    * keyType(M): 암호화 시 키 종류
        * 0: secp256k1
        * 1: secp256r1
        * **2: RSA**
    * authType(O): 인증방식
        * profile 내의 `authType`을 그대로 설정
        * presentType = 2일 때 생략
    * did(O): holder의 DID
        * presentType = 2일 때 생략
    * nonce(O): Profile의 nonce
        * presentType = 1일 때만 필수
    * zkpNonce(O): ZKP Profile의 nonce
        * presentType = 2일 때만 필수
    * type(M)
        * 'VERIFY'로 고정
    * data(M)
        * VP를 암호화하여 hexstring 인코딩한 값

[예시 1 - DID VP]
```json
{
  ...
  "vp": {
    "presentType": 1,
    "encryptType": 2,
    "keyType": 2,
    "authType":["pin"],
    "did": "did:omn:3WaMJYY4KeNsHmwH6G2Hmx8H9a74",
    "nonce": "2704b5e4eb5de74c3ff7956c53ae1dc9596a89545fe581755bee82995f7a83a2",
    "type": "VERIFY",
    "data": "01005666dd8098f6f825...c2b7"
  }
}
```

[예시 2 - ZKP(영지식증명) VP]
```json
{
  ...
  "vp": {
    "presentType": 2,
    "encryptType": 2,
    "keyType": 2,
    "zkpNonce": "96e0eaf9cdde350f0be124db726ec7175b81741761a1722a1c50d74eaaec2008",
    "type": "VERIFY",
    "data": "01005666dd8098f6f825...c2b7"
  }
}
```

**HTTP Request**
verifier에게 제출시는 해당 VP 제출메시지를 base64 인코딩해서 아래와 같이 전달한다.

```http
POST /mip/vp HTTP/1.1
Host:http://example.com
Content-type:application/json; charset=UTF-8

{
  "data":"base64 인코딩된 이미지 요청메시지"
}
```

**HTTP Response (정상)**
아래와 같이 응답한다.
```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":true
}
```

**HTTP Response (오류)**
오류 발생 시 아래와 같이 오류 메시지를 구성한다.

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"900",
  "trxcode":"20211102162732145432176790",
  "errcode":10002,
  "errmsg":"missing mandatory item: request"
}
```

http 응답코드와 함께 base64 인코딩된 오류 메시지를 응답한다.
```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":false,
  "data":"base64 인코딩된 오류 메시지"
}
```

[가능한 오류 목록]

| 오류코드<br>10진수  |             이름             |                               설명                               |
| ------------------ | --------------------------- | --------------------------------------------------------------- |
| 10001              | unexpected message format   | JSON 형식의 메시지가 아님                                         |
| 10002              | missing mandatory item      | type, version, cmd, request, trxcode, vp 및 vp 하위 필수항목 누락 |
| **--- 데이터 ---** |                             |                                                                 |
| 10100              | invalid data                | - request != 'presentation'<br>- vp.presentType != [0,1,2] 등    |
| 10101              | invalid type                | type != 'mip'                                                   |
| 10102              | unsupported message version | 지원하지 않는 메시지 버전                                          |
| 10103              | invalid cmd                 | cmd != '400'                                                    |
| 10104              | trxcode not found           | 존재하지 않는 거래코드                                             |
| **--- 절차 ---**   |                             |                                                                 |
| 10201              | message sequence error      | 선행 M310 송수신 필요                                             |
| 10202              | timeout error               | 유효시간 초과 오류                                                |
| **--- 기타 ---**   |                             |                                                                 |
| 99999              | unknown error               | 알 수 없는 오류                                                   |


## 2.5. 900 - 기타

### 2.5.1. [M900] 오류 메시지

오류 메시지는 다음 두 가지의 경우에 사용된다.

1. verifier가 신분증앱에 오류 응답
2. 신분증앱이 verifier에 오류 응답


```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"900",
  "trxcode":"20211102162732145432176790",
  "errcode":10002,
  "errmsg":"missing mandatory item: request"
}
```

[항목]
* errcode(M): 오류 코드
    * 오류 코드표 참조
* errmsg(M): 오류 메시지
    * 오류 코드표 참조


#### 2.5.1.1. verifier가 신분증앱에 오류 응답

http 응답의 data 항목에 오류 메시지를 base64로 인코딩하여 다음과 같이 응답한다.

```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
    "result":false,
    "data":"base64 인코딩된 오류 메시지"
}
```

#### 2.5.1.2. 신분증앱이 verifier에 오류 응답

**HTTP Request**
verifier에게 오류 전달 시 해당 메시지를 base64 인코딩해서 전달한다.

```http
POST /mip/error HTTP/1.1
Host:http://example.com
Content-type:application/json; charset=UTF-8

{
  "data":"base64 인코딩된 오류 메시지"
}
```

**HTTP Response**
verifier는 신분증앱으로부터 오류 메시지 수신 시 적절한 오류 처리 후 아래와 같은 메시지를 응답한다. 응답 후 verifier는 해당 거래코드의 세션을 종료한다. 신분증앱은 송신한 오류 메시지에 대한 verifier의 응답 여부 및 결과코드에 관계없이 해당 거래코드의 세션을 종료한다.

```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":true
}
```

# 3. 오류 코드

## 3.1. verifier 발생

| 오류코드<br>(10진수) |             이름             |              설명               |
| ------------------- | --------------------------- | ------------------------------ |
| **--- 형식 ---**    |                             |                                |
| 10001               | unexpected message format   | JSON 형식의 메시지가 아님        |
| 10002               | missing mandatory item      | 필수 항목 누락                   |
| 10003               | base64 decode error         | Base64 decoding error          |
| **--- 데이터 ---**  |                             |                                |
| 10100               | invalid data                | 유효하지 않은 항목 데이터         |
| 10101               | invalid type                | 유효하지 않은 type('mip'가 아님) |
| 10102               | unsupported message version | 지원하지 않는 메시지 버전         |
| 10103               | invalid cmd                 | 유효하지 않은 cmd                |
| 10104               | trxcode not found           | 존재하지 않는 거래코드            |
| 10105               | profile create error        | Profile 생성 오류               |
| 10106               | unsupported VP presentType  | 지원하지 않는 VP presentType     |
| 10107               | unsupported VP encryptType  | 지원하지 않는 VP encryptType     |
| 10108               | unsupported VP keyType      | 지원하지 않는 VP keyType         |
| 10109               | unsupported VP type         | 지원하지 않는 VP Type            |
| 10110               | unsupported VP authType     | 지원하지 않는 VP authType        |
| 10111               | mismatching nonce           | nonce 불일치                    |
| 10112               | mismatching authType        | authType 불일치                 |
| **--- 절차 ---**    |                             |                                |
| 10201               | message sequence error      | 메시지 전송 순서 오류            |
| 10202               | timeout error               | 유효시간 초과 오류               |
| **--- PUSH ---**    |                             |                                |
| 10301               | mscode is missing           | 미등록 연계시스템                |
| 10302               | pushType is missing         | pushType이 작성되지 않음         |
| 10303               | telno is missing            | telno가 작성되지 않음            |
| 10304               | data is missing             | data가 작성되지 않음             |
| 10305               | receive is missing          | 조회된 수신자 없음               |
| 10306               | insert data fail            | 메시지 정보 INSERT 실패          |
| 10399               | unknown error(PUSH)         | 알 수 없는 오류                  |
| **--- BLE ---**     |                             |                                |
| 10501               | incorrect password          | BLE 비밀번호 불일치              |
| **--- 기타 ---**    |                             |                                |
| 99999               | unknown error               | 알 수 없는 오류                  |


## 3.2. 모바일 신분증 발생

| 오류코드<br>(10진수)  |                이름                |                    설명                     |
| ------------------- | --------------------------------- | ------------------------------------------- |
| **--- 형식 ---**     |                                   |                                             |
| 20001               | unexpected message format         | JSON 형식의 메시지가 아님                     |
| 20002               | missing mandatory item            | 필수 항목 누락                               |
| **--- 데이터 ---**   |                                   |                                             |
| 20100               | invalid data                      | 유효하지 않은 항목 데이터                     |
| 20101               | invalid result                    | result가 true/false가 아님                   |
| 20102               | encoding error                    | 데이터가 정상적으로 인코딩되지 않음            |
| 20103               | mismatching trxcode               | 거래코드 불일치                              |
| 20104               | unsupported image type            | 지원하지 않는 이미지 타입                     |
| **--- 절차 ---**     |                                   |                                             |
| 20201               | message sequence error            | 메시지 전송 순서 오류                         |
| 20202               | timeout error                     | 유효시간 초과 오류                           |
| **--- Profile ---** |                                   |                                             |
| 20300               | invalid profile data              | profile 내 유효하지 않은 항목 데이터          |
| 20301               | VC not exists                     | 발급 받은 VC 없음                            |
| 20302               | no VC for allowed issuers         | 요청된 발급자용 VC 없음                       |
| 20303               | missing mandatory item in profile | profile 내 필수 항목 누락                    |
| 20304               | missing nonce in profile          | profile 내 nonce 없음                       |
| 20305               | missing proof in profile          | profile 내 proof 없음                       |
| 20306               | profile signature validation fail | profile 내 서명 검증 실패                    |
| 20307               | fail to get DID document          | DID document 조회 실패                       |
| **--- 영지식 ---**   |                                   |                                             |
| 20401               | ZKP error 1                       | 제출 정보 검색 실패(fail to search referent) |
| 20402               | ZKP error 2                       | 나이 조건 불일치                             |
| 20403               | ZKP error 3                       | Credential def, schema data 조회 실패        |
| **--- BLE ---**     |                                   |                                             |
| 20501               | incorrect password                | BLE 비밀번호 불일치                          |
| **--- 기타 ---**     |                                   |                                             |
| 20901               | cancel by user                    | 사용자에 의한 취소                           |
| 90001               | user authentication fail          | 사용자 인증 실패                             |
| 99999               | unknown error                     | 알 수 없는 오류                              |



<div style="page-break-after: always;"></div>

# 4. 요약

![인터페이스별, 단계별 메시지 요약표](vx_images/597241517220166.png)



※ 비고

* **M200(+/- profile)**: M200 메시지 내에 profile을 base64 string으로 포함여부가 옵션임을 의미
* **M200**: M310 메시지가 명시되어 있으면 M200 메시지에 profile 미포함


<div style="page-break-after: always;"></div>

# 5. 예시


## 5.1. QR-MPM direct mode

* SP서버의 호스트명(예시): `http://www.yourservice.com`
* 메시지 핸들러 URL
    * profile 조회: `http://www.yourservice.com/mip/profile`
    * image 조회: `http://www.yourservice.com/mip/image`
    * VP 제출: `http://www.yourservice.com/mip/vp`
    * 오류 제출: `http://www.yourservice.com/mip/error`

![QR-MPM direct mode 구성도](vx_images/254872211211250.png)


### 5.1.1. 절차

1. 응대장치가 SP서버에 QR정보 요청
    * 거래코드 생성: trxcode = "20211102162732145432176790"
    * profile 생성
    * profile은 `http://www.yourservice.com/mip/profile` 로 POST 요청 수신 시 제공함
2. **[M200]** QR 생성 및 표시
3. QR 촬영
4. 거래코드 기반 제출정보 전달
    * **[M310]** profile 수신
    * BI 이미지는 수신하지 않음
5. 제출정보 이용자 동의
6. 요청 인증방식으로 이용자 인증 수행
7. 암호키로 제출정보 암호화
8. **[M400]** 신원/자격정보 제출
9. 암호키 기반 제출정보 복호화
10. 신원/자격정보 검증
11. 서비스 제공

### 5.1.2. 상세

#### 5.1.2.1. 절차 2

**M200 VP 요청 메시지**
```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"200",
  "trxcode":"20211102162732145432176790",
  "mode":"direct",
  "ci":false,
  "host":"http://www.yourservice.com"
}
```

M200 메시지를 base64 인코딩 하여 QR을 생성한다.
```
ew0KICAidHlwZSI6Im1pcCIsDQogICJ2ZXJzaW9uIjoiMS4wLjAiLA0KICAiY21kIjoiMjAwIiwNCiAgInRyeGNvZGUiOiIyMDIxMTEwMjE2MjczMjE0NTQzMjE3Njc5MCIsDQogICJtb2RlIjoiZGlyZWN0IiwNCiAgImNpIjpmYWxzZSwNCiAgImhvc3QiOiJodHRwOi8vd3d3LnlvdXJzZXJ2aWNlLmNvbSINCn0
```

![M200 QR 코드](vx_images/411894910211270.png)

#### 5.1.2.2. 절차 4

M200 메시지에 profile 항목이 없으므로 host에 profile을 요청한다.

**M310 profile 요청 메시지**
```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"310",
  "request":"profile",
  "trxcode":"20211102162732145432176790"
}
```

**HTTP Request**
신분증앱은 `http://www.yourservice.com/mip/profile`로 아래 POST 메시지를 보내 profile을 요청한다.

```http
POST /mip/profile HTTP/1.1
Host:www.yourservice.com
Content-type:application/json; charset=UTF-8

{
  "data":"ewogICJ0eXBlIjoibWlwIiwKICAidmVyc2lvbiI6IjEuMC4wIiwKICAiY21kIjoiMzEwIiwKICAicmVxdWVzdCI6InByb2ZpbGUiLAogICJ0cnhjb2RlIjoiMjAyMTExMDIxNjI3MzIxNDU0MzIxNzY3OTAiCn0"
}
```

**HTTP Response**
SP서버는 trxcode로 DB를 검색하여 저장된 profile을 불러오고, 이를 base64 인코딩하여 아래 형태로 응답한다.

```http
HTTP/1.1 200
Content-type:application/json; charset=UTF-8

{
  "result":true,
  "data":"ew0KICAidHJ4Y29kZSI6IjIwMjExMTAyMTYyNzMyMTQ1NDMyMTc2NzkwIiwNCiAgInByb2ZpbGUiOiJldzBLSUNBZ0lDSmxibU52WkdsdVp5STZJQ0pWVkVZdE9DSXNEUW9nSUNBZ0ltbGtJam9nSW1ScFpEcHZiVzQ2TWtSelExZDZZVUZ1YUZobFVrNUVhMHRYU21SRk1tOTVjRkpTUWlJc0RRb2dJQ0FnSW14aGJtZDFZV2RsSWpvZ0lrdFNJaXdOQ2lBZ0lDQWljSEp2Wm1sc1pTSTZJSHNOQ2lBZ0lDQWdJQ0FnSW1GMWRHaFVlWEJsSWpvZ1d5SndhVzRpWFN3TkNna0pMaTR1RFFvSmZRMEtmUSINCn0"
}
```

#### 5.1.2.3. 절차 8

신분증앱은 profile을 입수하여 사용자 인증 등 후속절차 진행 후, VP 생성, VP 암호화 등을 수행하여 M400 메시지를 생성한다.

**M400 VP 제출 메시지**

vp 항목의 용량이 크기 때문에 상세 값은 생략함

```json
{
  "type":"mip",
  "version":"1.0.0",
  "cmd":"400",
  "request":"presentation",
  "trxcode":"20211102162732145432176790",
  "vp":{...}
}
```

**HTTP Request**
신분증앱은 `http://www.yourservice.com/mip/vp`로 아래 POST 메시지를 보내 VP를 제출한다.

```http
POST /mip/vp HTTP/1.1
Host:www.yourservice.com
Content-type:application/json; charset=UTF-8

{
  "data":"ew0KICAidHlwZSI6Im1pcCIsDQogICJ2ZXJzaW9uIjoiMS4wLjAiLA0KICAiY21kIjoiMzEwIiwNCiAgInJlcXVlc3QiOiJwcm9maWxlIiwNCiAgInRyeGNvZGUiOiIyMDIxMTEwMjE2MjczMjE0NTQzMjE3Njc5MCINCn0"
}
```

<div style="page-break-after: always;"></div>

# 6. 부록

## 6.1. BLE 인터페이스 메시지

### 6.1.1. password
BLE 연결을 위해 비밀번호를 제출하고 검증한다.

**[메시지 예시]**

```json
{
  "msg":"password",
  "trxcode":"20211102162732145432176790",
  "value":"0123456789"
}
```

**[항목]**

* trxcode(M): 거래코드
* blepw(M): 비밀번호

### 6.1.2. ack
이전 메시지를 정상적으로 수신하거나 처리하였음을 알린다.


**[메시지 예시]**
```json
{
  "msg":"ack",
  "trxcode":"20211102162732145432176790"
}
```

**[항목]**

* trxcode(M): 거래코드

### 6.1.3. profile
Verifier가 생성한 profile을 전달한다.

**[메시지 예시]**

```json
{
  "msg":"profile",
  "trxcode":"20211102162732145432176790",
  "profile":"base64 인코딩된 profile",
  "image":"base64 인코딩된 BI 이미지",
  "ci":false
}
```

**[항목]**

* trxcode(M): 거래코드
* profile(M): profile string을 base64 인코딩한 값
* image(O):
    * BI 이미지를 base64 인코딩한 값
    * null: 이미지 없음
* ci(O): ci 제출 여부
    * true: VP에 CI를 포함
    * false/null: VP에 CI를 포함하지 않음

### 6.1.4. vp
신분증앱이 profile을 참조해 생성한 VP를 전달한다.

**[메시지 예시]**

```json
{
  "msg":"vp",
  "trxcode":"20211102162732145432176790",
  "request":"presentation",
  "vp":{...}
}
```

**[항목]**

* trxcode(M): 거래코드
* request(M): 'presentation' 고정
* vp(M)
    * M400.vp 참조

### 6.1.5. finish
VP 제출이 최종 성공하였음을 응답한다.

**[메시지 예시]**

```json
{
  "msg":"finish",
  "trxcode":"20211102162732145432176790"
}
```

**[항목]**

* trxcode(M): 거래코드

### 6.1.6. error
오류 내용을 응답한다.

**[메시지 예시]**

```json
{
  "msg":"error",
  "trxcode":"20211102162732145432176790",
  "errcode":20902,
  "errmsg":"invalid password"
}
```

**[항목]**

* trxcode(M): 거래코드
* errcode(M): 에러코드
* errmsg(M): 에러 내용


<div style="page-break-after: always;"></div>


# 7. Release Notes

## 7.1. v1.6.3: 2021-12-17

* 2.4.1. [M400] VP 제출 메시지
    * present 항목 삭제하고 vp 항목의 하위항목으로 presentType, authType 추가


## 7.2. v1.7.0: 2021-12-29

* 2.1.1. QR-CPM용 전처리 메시지
    * Base64 및 QR코드 예시 추가
* 2.2.2.1. QR-MPM
    * 주의사항 추가
* 2.2.2.2. PUSH
    * 응답 메시지 및 발생 가능한 오류코드 추가
* 2.3.1. [M310] Profile 요청 메시지
    * HTTP Response에 result 항목 추가
    * 발생 가능한 오류코드 추가
* 2.3.2. [M320] Image 요청 메시지
    * HTTP Response에 result 항목 추가
    * 발생 가능한 오류코드 추가
* 2.4.1. [M400] VP 제출 메시지
    * vp.zkpNonce 추가 (영지식 전용 nonce)
    * HTTP Response에 result 항목 추가
    * 발생 가능한 오류코드 추가
* 2.5. 900 - 기타 메시지
    * 신규 추가
    * 오류 메시지 handler `mip/error` 추가
* **3. 오류 코드**
    * 신규 추가

## 7.3. v1.7.1: 2022-01-12

* 2.2.2.2. PUSH
    * 정상 응답 메시지 포맷 수정
    * 오류 응답 메시지 포맷 수정
    * 오류 코드 추가
* 3.1. verifier 발생
    * 오류 코드 추가
* 3.2. 모바일 신분증 발생
    * 오류 코드 추가: 20901

## 7.4. v1.7.2: 2022-01-26

* 1.3.1. Base64
    * Java 예제 코드 수정
        * `Base64.getUrlEncoder().encode(...)` => `Base64.getUrlEncoder().withoutPadding().encode(...)`
* 2.2.2.2. PUSH
    * "PUSH 요청 메시지"의 `data` 항목 설명 추가
    * "PUSH용 M200 예시" 추가
        * M200에 profile을 base64로 인코딩하여 포함할 경우 용량 문제로 PUSH 발송이 실패할 수 있음
        * 그러므로 profile을 미포함하여 *host*/mip/profile 주소로 M310 메시지를 전송하도록 유도
* '4. 요약
    * PUSH direct
        * M200 (+/- profile) => M200
        * M310(profile 없을때) => M310


## 7.5. v1.8.0: 2022-05-25

* 2.2.2.2. PUSH
    * 아래 문구 삭제
        * "아래 PUSH 전송 요청문의 `data`를 base64 인코딩 시 반드시 <span style="color:red">**padding**</span>을 적용하여야 한다."
    * 즉, 패딩을 하지 않아도 됨
* 2.2.2.4. Bluetooth
    * 제목을 Bluetooth에서 BLE로 변경
    * BLE 인터페이스 전송방법 설명 추가
* 3.1. verifier 발생
    * BLE 오류코드 추가
* 3.2. 모바일 신분증 발생
    * BLE 오류코드 추가
* '6. 부록
    * 신규 추가
    * BLE 인터페이스 메시지 추가
