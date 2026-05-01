#[doc = "Register `DEV00` reader"]
pub type R = crate::R<Dev00Spec>;
#[doc = "Register `DEV00` writer"]
pub type W = crate::W<Dev00Spec>;
#[doc = "Field `EnblDevPort` reader - Enable device port"]
pub type EnblDevPortR = crate::BitReader;
#[doc = "Field `EnblDevPort` writer - Enable device port"]
pub type EnblDevPortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DevPortSpeedSel` reader - Device port speed selection"]
pub type DevPortSpeedSelR = crate::BitReader;
#[doc = "Field `DevPortSpeedSel` writer - Device port speed selection"]
pub type DevPortSpeedSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblEndpoint0SETUPDataPktACKINT` reader - Enable Endpoint 0 SETUP data packet ACK interrupt"]
pub type EnblEndpoint0setupdataPktAckintR = crate::BitReader;
#[doc = "Field `EnblEndpoint0SETUPDataPktACKINT` writer - Enable Endpoint 0 SETUP data packet ACK interrupt"]
pub type EnblEndpoint0setupdataPktAckintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblEndpoint0OUTDataPktACKSTALLINT` reader - Enable Endpoint 0 OUT data packet ACK/STALL interrupt"]
pub type EnblEndpoint0outdataPktAckstallintR = crate::BitReader;
#[doc = "Field `EnblEndpoint0OUTDataPktACKSTALLINT` writer - Enable Endpoint 0 OUT data packet ACK/STALL interrupt"]
pub type EnblEndpoint0outdataPktAckstallintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblEndpoint0OUTDataPktNAKINT` reader - Enable Endpoint 0 OUT data packet NAK interrupt"]
pub type EnblEndpoint0outdataPktNakintR = crate::BitReader;
#[doc = "Field `EnblEndpoint0OUTDataPktNAKINT` writer - Enable Endpoint 0 OUT data packet NAK interrupt"]
pub type EnblEndpoint0outdataPktNakintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblEndpoint0INDataPktACKSTALLINT` reader - Enable Endpoint 0 IN data packet ACK/STALL interrupt"]
pub type EnblEndpoint0indataPktAckstallintR = crate::BitReader;
#[doc = "Field `EnblEndpoint0INDataPktACKSTALLINT` writer - Enable Endpoint 0 IN data packet ACK/STALL interrupt"]
pub type EnblEndpoint0indataPktAckstallintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblEndpoint0INDataPktNAKINT` reader - Enable Endpoint 0 IN data packet NAK interrupt"]
pub type EnblEndpoint0indataPktNakintR = crate::BitReader;
#[doc = "Field `EnblEndpoint0INDataPktNAKINT` writer - Enable Endpoint 0 IN data packet NAK interrupt"]
pub type EnblEndpoint0indataPktNakintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `DownstreamDevAddr` reader - Downstream Device Address"]
pub type DownstreamDevAddrR = crate::FieldReader;
#[doc = "Field `DownstreamDevAddr` writer - Downstream Device Address"]
pub type DownstreamDevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Enable device port"]
    #[inline(always)]
    pub fn enbl_dev_port(&self) -> EnblDevPortR {
        EnblDevPortR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device port speed selection"]
    #[inline(always)]
    pub fn dev_port_speed_sel(&self) -> DevPortSpeedSelR {
        DevPortSpeedSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Endpoint 0 SETUP data packet ACK interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0setupdata_pkt_ackint(&self) -> EnblEndpoint0setupdataPktAckintR {
        EnblEndpoint0setupdataPktAckintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Endpoint 0 OUT data packet ACK/STALL interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0outdata_pkt_ackstallint(&self) -> EnblEndpoint0outdataPktAckstallintR {
        EnblEndpoint0outdataPktAckstallintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Endpoint 0 OUT data packet NAK interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0outdata_pkt_nakint(&self) -> EnblEndpoint0outdataPktNakintR {
        EnblEndpoint0outdataPktNakintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Endpoint 0 IN data packet ACK/STALL interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0indata_pkt_ackstallint(&self) -> EnblEndpoint0indataPktAckstallintR {
        EnblEndpoint0indataPktAckstallintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Endpoint 0 IN data packet NAK interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0indata_pkt_nakint(&self) -> EnblEndpoint0indataPktNakintR {
        EnblEndpoint0indataPktNakintR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Downstream Device Address"]
    #[inline(always)]
    pub fn downstream_dev_addr(&self) -> DownstreamDevAddrR {
        DownstreamDevAddrR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Enable device port"]
    #[inline(always)]
    pub fn enbl_dev_port(&mut self) -> EnblDevPortW<Dev00Spec> {
        EnblDevPortW::new(self, 0)
    }
    #[doc = "Bit 1 - Device port speed selection"]
    #[inline(always)]
    pub fn dev_port_speed_sel(&mut self) -> DevPortSpeedSelW<Dev00Spec> {
        DevPortSpeedSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Endpoint 0 SETUP data packet ACK interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0setupdata_pkt_ackint(
        &mut self,
    ) -> EnblEndpoint0setupdataPktAckintW<Dev00Spec> {
        EnblEndpoint0setupdataPktAckintW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Endpoint 0 OUT data packet ACK/STALL interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0outdata_pkt_ackstallint(
        &mut self,
    ) -> EnblEndpoint0outdataPktAckstallintW<Dev00Spec> {
        EnblEndpoint0outdataPktAckstallintW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Endpoint 0 OUT data packet NAK interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0outdata_pkt_nakint(
        &mut self,
    ) -> EnblEndpoint0outdataPktNakintW<Dev00Spec> {
        EnblEndpoint0outdataPktNakintW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Endpoint 0 IN data packet ACK/STALL interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0indata_pkt_ackstallint(
        &mut self,
    ) -> EnblEndpoint0indataPktAckstallintW<Dev00Spec> {
        EnblEndpoint0indataPktAckstallintW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Endpoint 0 IN data packet NAK interrupt"]
    #[inline(always)]
    pub fn enbl_endpoint0indata_pkt_nakint(&mut self) -> EnblEndpoint0indataPktNakintW<Dev00Spec> {
        EnblEndpoint0indataPktNakintW::new(self, 6)
    }
    #[doc = "Bits 8:14 - Downstream Device Address"]
    #[inline(always)]
    pub fn downstream_dev_addr(&mut self) -> DownstreamDevAddrW<Dev00Spec> {
        DownstreamDevAddrW::new(self, 8)
    }
}
#[doc = "Downstream Device Function Enable Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev00Spec;
impl crate::RegisterSpec for Dev00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev00::R`](R) reader structure"]
impl crate::Readable for Dev00Spec {}
#[doc = "`write(|w| ..)` method takes [`dev00::W`](W) writer structure"]
impl crate::Writable for Dev00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV00 to value 0"]
impl crate::Resettable for Dev00Spec {}
