<div class="WordSection1">

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901000">1<span style="font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif;color:windowtext">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; </span>Introduction<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">11</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901001">1.0
Intellectual property rights policy<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">11</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901002">1.1
Organization of the MQTT specification<span style="color:windowtext;display:
none">. </span><span style="color:windowtext;display:none">11</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901003">1.2
Terminology<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">11</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901004">1.3 Normative
references<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">13</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901005">1.4
Non-normative references<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">13</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901006">1.5 Data
representation<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">16</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901007">1.5.1 Bits<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">16</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901008">1.5.2 Two Byte
Integer<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">16</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901009">1.5.3 Four
Byte Integer<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">16</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901010">1.5.4 UTF-8
Encoded String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">16</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901011">1.5.5 Variable
Byte Integer<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">18</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901012">1.5.6 Binary
Data<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">19</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901013">1.5.7 UTF-8
String Pair<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">19</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901014">1.6 Security<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">19</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901015">1.7 Editing
convention<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">20</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901016">1.8 Change
history<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">20</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901017">1.8.1 MQTT
v3.1.1<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">20</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901018">1.8.2 MQTT
v5.0<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">20</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901019">2<span style="font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif;color:windowtext">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; </span>MQTT
Control Packet format<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">21</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901020">2.1 Structure
of an MQTT Control Packet<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">21</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901021">2.1.1 Fixed
Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">21</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901022">2.1.2 MQTT
Control Packet type<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">21</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901023">2.1.3 Flags<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">22</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901024">2.1.4
Remaining Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">23</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901025">2.2 Variable
Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">23</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901026">2.2.1 Packet
Identifier<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">23</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901027">2.2.2
Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">25</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901028">2.2.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">25</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901029">2.2.2.2
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">25</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901030">2.3 Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">26</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901031">2.4 Reason
Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">27</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901032">3<span style="font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif;color:windowtext">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; </span>MQTT
Control Packets<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">30</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901033">3.1 CONNECT –
Connection Request<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">30</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901034">3.1.1 CONNECT
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">30</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901035">3.1.2 CONNECT
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">30</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901036">3.1.2.1
Protocol Name<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">30</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901037">3.1.2.2
Protocol Version<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">31</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901038">3.1.2.3
Connect Flags<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">31</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901039">3.1.2.4 Clean
Start<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">32</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901040">3.1.2.5 Will
Flag<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">32</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901041">3.1.2.6 Will
QoS<span style="color:windowtext;display:none">.. </span><span style="color:windowtext;display:none">33</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901042">3.1.2.7 Will
Retain<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">33</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901043">3.1.2.8 User
Name Flag<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">33</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901044">3.1.2.9
Password Flag<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">33</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901045">3.1.2.10 Keep
Alive<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">34</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901046">3.1.2.11
CONNECT Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">34</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901047">3.1.2.11.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">34</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901048"><span lang="EN-GB">3.1.2.11.2 Session Expiry Interval</span><span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">35</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901049">3.1.2.11.3
Receive Maximum<span style="color:windowtext;display:none">... </span><span style="color:windowtext;display:none">36</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901050">3.1.2.11.4
Maximum Packet Size<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">36</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901051">3.1.2.11.5
Topic Alias Maximum<span style="color:windowtext;display:none">... </span><span style="color:windowtext;display:none">37</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901052">3.1.2.11.6
Request Response Information<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">37</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901053">3.1.2.11.7
Request Problem Information<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">37</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901054">3.1.2.11.8
User Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">38</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901055">3.1.2.11.9
Authentication Method<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">38</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901056">3.1.2.11.10
Authentication Data<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">38</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901057">3.1.2.12
Variable Header non-normative example<span style="color:windowtext;display:
none">. </span><span style="color:windowtext;display:none">39</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901058">3.1.3 CONNECT
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">40</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901059">3.1.3.1 Client
Identifier (ClientID)<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">40</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901060">3.1.3.2 Will
Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">40</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901061">3.1.3.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">40</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901062">3.1.3.2.2 Will
Delay Interval<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">41</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901063">3.1.3.2.3
Payload Format Indicator<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">41</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901064">3.1.3.2.4
Message Expiry Interval<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">41</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901065">3.1.3.2.5
Content Type<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">42</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901066">3.1.3.2.6
Response Topic<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">42</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901067">3.1.3.2.7
Correlation Data<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">42</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901068">3.1.3.2.8 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">42</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901069">3.1.3.3 Will
Topic<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">42</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901070">3.1.3.4 Will
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">43</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901071">3.1.3.5 User
Name<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">43</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901072">3.1.3.6
Password<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">43</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901073">3.1.4 CONNECT
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">43</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901074">3.2 CONNACK –
Connect acknowledgement<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">44</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901075">3.2.1 CONNACK
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">45</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901076">3.2.2 CONNACK
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">45</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901077">3.2.2.1
Connect Acknowledge Flags<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">45</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901078">3.2.2.1.1
Session Present<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">45</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901079">3.2.2.2
Connect Reason Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">46</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901080">3.2.2.3
CONNACK Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">47</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901081">3.2.2.3.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">47</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901082"><span lang="EN-GB">3.2.2.3.2 Session Expiry Interval</span><span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">47</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901083">3.2.2.3.3
Receive Maximum<span style="color:windowtext;display:none">... </span><span style="color:windowtext;display:none">48</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901084">3.2.2.3.4
Maximum QoS<span style="color:windowtext;display:none">.. </span><span style="color:windowtext;display:none">48</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901085">3.2.2.3.5
Retain Available<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">49</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901086">3.2.2.3.6
Maximum Packet Size<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">49</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901087">3.2.2.3.7
Assigned Client Identifier<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">49</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901088">3.2.2.3.8
Topic Alias Maximum<span style="color:windowtext;display:none">... </span><span style="color:windowtext;display:none">50</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901089">3.2.2.3.9
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">50</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901090">3.2.2.3.10
User Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">50</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901091">3.2.2.3.11
Wildcard Subscription Available<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">50</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901092">3.2.2.3.12
Subscription Identifiers Available<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">51</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901093">3.2.2.3.13
Shared Subscription Available<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">51</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901094">3.2.2.3.14
Server Keep Alive<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">51</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901095">3.2.2.3.15
Response Information<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">52</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901096">3.2.2.3.16
Server Reference<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">52</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901097">3.2.2.3.17
Authentication Method<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">52</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901098">3.2.2.3.18
Authentication Data<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">52</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901099">3.2.3 CONNACK
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">53</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901100">3.3 PUBLISH –
Publish message<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">53</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901101">3.3.1 PUBLISH
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">53</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901102">3.3.1.1 DUP<span style="color:windowtext;display:none">.. </span><span style="color:windowtext;display:none">53</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901103">3.3.1.2 QoS<span style="color:windowtext;display:none">.. </span><span style="color:windowtext;display:none">54</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901104">3.3.1.3 RETAIN<span style="color:windowtext;display:none">.. </span><span style="color:windowtext;display:none">54</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901105">3.3.1.4
Remaining Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">55</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901106">3.3.2 PUBLISH
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">55</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901107">3.3.2.1 Topic
Name<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">55</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901108">3.3.2.2 Packet
Identifier<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">56</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901109">3.3.2.3
PUBLISH Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">56</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901110">3.3.2.3.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">56</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901111">3.3.2.3.2
Payload Format Indicator<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">56</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901112">3.3.2.3.3
Message Expiry Interval`<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">56</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901113">3.3.2.3.4
Topic Alias<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">57</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901114">3.3.2.3.5
Response Topic<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">58</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901115">3.3.2.3.6
Correlation Data<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">58</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901116">3.3.2.3.7 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">58</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901117">3.3.2.3.8
Subscription Identifier<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">59</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901118">3.3.2.3.9
Content Type<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">59</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901119">3.3.3 PUBLISH
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">60</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901120">3.3.4 PUBLISH
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">60</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901121">3.4 PUBACK –
Publish acknowledgement<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">62</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901122">3.4.1 PUBACK
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">63</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901123">3.4.2 PUBACK
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">63</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901124">3.4.2.1 PUBACK
Reason Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">63</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901125">3.4.2.2 PUBACK
Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">64</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901126">3.4.2.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">64</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901127">3.4.2.2.2
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">64</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901128">3.4.2.2.3 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">64</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901129">3.4.3 PUBACK
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">65</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901130">3.4.4 PUBACK
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">65</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901131">3.5 PUBREC –
Publish received (QoS 2 delivery part 1)<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">65</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901132">3.5.1 PUBREC
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">65</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901133">3.5.2 PUBREC
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">65</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901134">3.5.2.1 PUBREC
Reason Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">65</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901135">3.5.2.2 PUBREC
Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">66</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901136">3.5.2.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">66</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901137">3.5.2.2.2
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">66</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901138">3.5.2.2.3 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">67</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901139">3.5.3 PUBREC
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">67</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901140">3.5.4 PUBREC
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">67</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901141">3.6 PUBREL –
Publish release (QoS 2 delivery part 2)<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">67</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901142">3.6.1 PUBREL
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">67</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901143">3.6.2 PUBREL
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">67</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901144">3.6.2.1 PUBREL
Reason Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">68</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901145">3.6.2.2 PUBREL
Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">68</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901146">3.6.2.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">68</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901147">3.6.2.2.2
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">68</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901148">3.6.2.2.3 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">69</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901149">3.6.3 PUBREL
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">69</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901150">3.6.4 PUBREL
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">69</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901151">3.7 PUBCOMP –
Publish complete (QoS 2 delivery part 3)<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">69</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901152">3.7.1 PUBCOMP
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">69</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901153">3.7.2 PUBCOMP
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">69</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901154">3.7.2.1
PUBCOMP Reason Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">70</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901155">3.7.2.2
PUBCOMP Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">70</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901156">3.7.2.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">70</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901157">3.7.2.2.2
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">70</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901158">3.7.2.2.3 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">70</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901159">3.7.3 PUBCOMP
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">71</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901160">3.7.4 PUBCOMP
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">71</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901161">3.8 SUBSCRIBE
- Subscribe request<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">71</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901162">3.8.1
SUBSCRIBE Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">71</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901163">3.8.2
SUBSCRIBE Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">71</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901164">3.8.2.1
SUBSCRIBE Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">72</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901165">3.8.2.1.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">72</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901166">3.8.2.1.2
Subscription Identifier<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">72</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901167">3.8.2.1.3 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">72</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901168">3.8.3
SUBSCRIBE Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">72</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901169">3.8.3.1
Subscription Options<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">73</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901170">3.8.4 SUBSCRIBE
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">75</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901171">3.9 SUBACK –
Subscribe acknowledgement<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">77</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901172">3.9.1 SUBACK
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">77</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901173">3.9.2 SUBACK
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">77</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901174">3.9.2.1 SUBACK
Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">77</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901175">3.9.2.1.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">77</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901176">3.9.2.1.2
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">78</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901177">3.9.2.1.3 User
Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">78</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901178">3.9.3 SUBACK
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">78</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901179">3.10
UNSUBSCRIBE – Unsubscribe request<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">79</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901180">3.10.1
UNSUBSCRIBE Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">79</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901181">3.10.2
UNSUBSCRIBE Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">80</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901182">3.10.2.1
UNSUBSCRIBE Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">80</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901183">3.10.2.1.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">80</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901184">3.10.2.1.2
User Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">80</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901185">3.10.3
UNSUBSCRIBE Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">80</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901186">3.10.4
UNSUBSCRIBE Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">81</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901187">3.11 UNSUBACK
– Unsubscribe acknowledgement<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">81</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901188">3.11.1
UNSUBACK Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">82</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901189">3.11.2
UNSUBACK Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">82</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901190">3.11.2.1
UNSUBACK Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">82</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901191">3.11.2.1.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">82</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901192">3.11.2.1.2
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">82</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901193">3.11.2.1.3
User Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">83</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901194">3.11.3
UNSUBACK Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">83</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901195">3.12 PINGREQ –
PING request<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">83</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901196">3.12.1 PINGREQ
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">84</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901197">3.12.2 PINGREQ
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">84</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901198">3.12.3 PINGREQ
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">84</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901199">3.12.4 PINGREQ
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">84</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901200">3.13 PINGRESP
– PING response<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">84</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901201">3.13.1
PINGRESP Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">84</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901202">3.13.2
PINGRESP Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">85</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901203">3.13.3
PINGRESP Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">85</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901204">3.13.4
PINGRESP Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">85</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901205">3.14
DISCONNECT – Disconnect notification<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">85</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901206">3.14.1
DISCONNECT Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">85</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901207">3.14.2
DISCONNECT Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">85</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901208">3.14.2.1
Disconnect Reason Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">86</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901209">3.14.2.2
DISCONNECT Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">88</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901210">3.14.2.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">88</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901211">3.14.2.2.2
Session Expiry Interval<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">88</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901212">3.14.2.2.3
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">88</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901213">3.14.2.2.4
User Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">88</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901214">3.14.2.2.5
Server Reference<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">88</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901215">3.14.3 DISCONNECT
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">89</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901216">3.14.4
DISCONNECT Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">89</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901217">3.15 AUTH –
Authentication exchange<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">89</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901218">3.15.1 AUTH
Fixed Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">90</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901219">3.15.2 AUTH
Variable Header<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">90</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901220">3.15.2.1
Authenticate Reason Code<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">90</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901221">3.15.2.2 AUTH
Properties<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">90</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901222">3.15.2.2.1
Property Length<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">90</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901223">3.15.2.2.2
Authentication Method<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">91</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901224">3.15.2.2.3
Authentication Data<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">91</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901225">3.15.2.2.4
Reason String<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">91</span></a></span></p>

<p class="MsoToc5"><span class="MsoHyperlink"><a href="#_Toc3901226">3.15.2.2.5
User Property<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">91</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901227">3.15.3 AUTH
Payload<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">91</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901228">3.15.4 AUTH
Actions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">91</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901229">4<span style="font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif;color:windowtext">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; </span>Operational
behavior<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">92</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901230">4.1 Session
State<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">92</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901231">4.1.1 Storing
Session State<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">92</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901232">4.1.2 Session
State non-normative examples<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">93</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901233">4.2 Network
Connections<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">93</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901234">4.3 Quality of
Service levels and protocol flows<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">93</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901235">4.3.1 QoS 0:
At most once delivery<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">94</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901236">4.3.2 QoS 1:
At least once delivery<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">94</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901237">4.3.3 QoS 2:
Exactly once delivery<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">95</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901238">4.4 Message
delivery retry<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">96</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901239">4.5 Message
receipt<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">97</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901240">4.6 Message
ordering<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">97</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901241">4.7 Topic
Names and Topic Filters<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">98</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901242">4.7.1 Topic
wildcards<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">98</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901243">4.7.1.1 Topic
level separator<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">98</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901244">4.7.1.2
Multi-level wildcard<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">98</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901245">4.7.1.3
Single-level wildcard<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">99</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901246">4.7.2 Topics
beginning with $<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">99</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901247">4.7.3 Topic
semantic and usage<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">100</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901248">4.8
Subscriptions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">101</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901249">4.8.1 Non‑shared
Subscriptions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">101</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901250">4.8.2 Shared
Subscriptions<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">101</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901251">4.9 Flow Control<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">103</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901252">4.10 Request /
Response<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">104</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901253">4.10.1 Basic
Request Response (non-normative)<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">104</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901254">4.10.2
Determining a Response Topic value (non-normative)<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">105</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901255">4.11 Server
redirection<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">106</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901256">4.12 Enhanced
authentication<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">106</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901257">4.12.1
Re-authentication<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">108</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901258">4.13 Handling
errors<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">109</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901259">4.13.1
Malformed Packet and Protocol Errors<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">109</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901260">4.13.2 Other
errors<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">110</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901261">5<span style="font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif;color:windowtext">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; </span>Security
(non-normative)<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">111</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901262">5.1
Introduction<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">111</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901263">5.2 MQTT
solutions: security and certification<span style="color:windowtext;display:
none">. </span><span style="color:windowtext;display:none">111</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901264">5.3
Lightweight crytography and constrained devices<span style="color:windowtext;
display:none">. </span><span style="color:windowtext;display:none">112</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901265">5.4
Implementation notes<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">112</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901266">5.4.1
Authentication of Clients by the Server<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">112</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901267">5.4.2
Authorization of Clients by the Server<span style="color:windowtext;display:
none"> </span><span style="color:windowtext;display:none">112</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901268">5.4.3
Authentication of the Server by the Client<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">113</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901269">5.4.4
Integrity of Application Messages and MQTT Control Packets<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">113</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901270">5.4.5 Privacy
of Application Messages and MQTT Control Packets<span style="color:windowtext;
display:none">. </span><span style="color:windowtext;display:none">113</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901271">5.4.6
Non-repudiation of message transmission<span style="color:windowtext;
display:none">. </span><span style="color:windowtext;display:none">114</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901272">5.4.7
Detecting compromise of Clients and Servers<span style="color:windowtext;
display:none">. </span><span style="color:windowtext;display:none">114</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901273">5.4.8
Detecting abnormal behaviors<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">114</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901274">5.4.9 Handling
of Disallowed Unicode code points<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">115</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901275">5.4.9.1
Considerations for the use of Disallowed Unicode code points<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">115</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901276">5.4.9.2
Interactions between Publishers and Subscribers<span style="color:windowtext;
display:none">. </span><span style="color:windowtext;display:none">115</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901277">5.4.9.3
Remedies<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">116</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901278">5.4.10 Other
security considerations<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">116</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901279">5.4.11 Use of
SOCKS<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">116</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901280">5.4.12
Security profiles<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">117</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901281">5.4.12.1 Clear
communication profile<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">117</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901282">5.4.12.2
Secured network communication profile<span style="color:windowtext;display:
none">. </span><span style="color:windowtext;display:none">117</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901283">5.4.12.3
Secured transport profile<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">117</span></a></span></p>

<p class="MsoToc4"><span class="MsoHyperlink"><a href="#_Toc3901284">5.4.12.4
Industry specific security profiles<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">117</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901285">6<span style="font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif;color:windowtext">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; </span>Using
WebSocket as a network transport<span style="color:windowtext;display:none"> </span><span style="color:windowtext;display:none">118</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901286">6.1 IANA
considerations<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">118</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901287">7<span style="font-size:11.0pt;font-family:&quot;Calibri&quot;,sans-serif;color:windowtext">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; </span>Conformance<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">119</span></a></span></p>

<p class="MsoToc2"><span class="MsoHyperlink"><a href="#_Toc3901288">7.1
Conformance clauses<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">119</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901289">7.1.1 MQTT
Server conformance clause<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">119</span></a></span></p>

<p class="MsoToc3"><span class="MsoHyperlink"><a href="#_Toc3901290">7.1.2 MQTT
Client conformance clause<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">119</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901291">Appendix A.
Acknowledgments<span style="color:windowtext;display:none">. </span><span style="color:windowtext;display:none">120</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901292">Appendix B.
Mandatory normative statement (non-normative)<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">121</span></a></span></p>

<p class="MsoToc1"><span class="MsoHyperlink"><a href="#_Toc3901293">Appendix C.
Summary of new features in MQTT v5.0 (non-normative)<span style="color:windowtext;
display:none"> </span><span style="color:windowtext;display:none">136</span></a></span></p>

<p class="TextBody">&nbsp;</p>

<p class="TextBody">&nbsp;</p>

</div>
