#[doc = "Register `DEV04` reader"]
pub type R = crate::R<Dev04Spec>;
#[doc = "Register `DEV04` writer"]
pub type W = crate::W<Dev04Spec>;
#[doc = "Field `Endpoint0SETUPDataPktRxd` reader - Endpoint 0 SETUP data packet received"]
pub type Endpoint0setupdataPktRxdR = crate::BitReader;
#[doc = "Field `Endpoint0SETUPDataPktRxd` writer - Endpoint 0 SETUP data packet received"]
pub type Endpoint0setupdataPktRxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Endpoint0OUTDataPktACKSTALLReturned` reader - Endpoint 0 OUT data packet ACK/STALL returned"]
pub type Endpoint0outdataPktAckstallreturnedR = crate::BitReader;
#[doc = "Field `Endpoint0OUTDataPktACKSTALLReturned` writer - Endpoint 0 OUT data packet ACK/STALL returned"]
pub type Endpoint0outdataPktAckstallreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Endpoint0OUTDataPktNAKReturned` reader - Endpoint 0 OUT data packet NAK returned"]
pub type Endpoint0outdataPktNakreturnedR = crate::BitReader;
#[doc = "Field `Endpoint0OUTDataPktNAKReturned` writer - Endpoint 0 OUT data packet NAK returned"]
pub type Endpoint0outdataPktNakreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Endpoint0INDataPktACKRxdOrSTALLReturned` reader - Endpoint 0 IN data packet ACK received or STALL returned"]
pub type Endpoint0indataPktAckrxdOrStallreturnedR = crate::BitReader;
#[doc = "Field `Endpoint0INDataPktACKRxdOrSTALLReturned` writer - Endpoint 0 IN data packet ACK received or STALL returned"]
pub type Endpoint0indataPktAckrxdOrStallreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Endpoint0INDataPktNAKReturned` reader - Endpoint 0 IN data packet NAK returned"]
pub type Endpoint0indataPktNakreturnedR = crate::BitReader;
#[doc = "Field `Endpoint0INDataPktNAKReturned` writer - Endpoint 0 IN data packet NAK returned"]
pub type Endpoint0indataPktNakreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 SETUP data packet received"]
    #[inline(always)]
    pub fn endpoint0setupdata_pkt_rxd(&self) -> Endpoint0setupdataPktRxdR {
        Endpoint0setupdataPktRxdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 0 OUT data packet ACK/STALL returned"]
    #[inline(always)]
    pub fn endpoint0outdata_pkt_ackstallreturned(&self) -> Endpoint0outdataPktAckstallreturnedR {
        Endpoint0outdataPktAckstallreturnedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 0 OUT data packet NAK returned"]
    #[inline(always)]
    pub fn endpoint0outdata_pkt_nakreturned(&self) -> Endpoint0outdataPktNakreturnedR {
        Endpoint0outdataPktNakreturnedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 0 IN data packet ACK received or STALL returned"]
    #[inline(always)]
    pub fn endpoint0indata_pkt_ackrxd_or_stallreturned(
        &self,
    ) -> Endpoint0indataPktAckrxdOrStallreturnedR {
        Endpoint0indataPktAckrxdOrStallreturnedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 0 IN data packet NAK returned"]
    #[inline(always)]
    pub fn endpoint0indata_pkt_nakreturned(&self) -> Endpoint0indataPktNakreturnedR {
        Endpoint0indataPktNakreturnedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 SETUP data packet received"]
    #[inline(always)]
    pub fn endpoint0setupdata_pkt_rxd(&mut self) -> Endpoint0setupdataPktRxdW<Dev04Spec> {
        Endpoint0setupdataPktRxdW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 0 OUT data packet ACK/STALL returned"]
    #[inline(always)]
    pub fn endpoint0outdata_pkt_ackstallreturned(
        &mut self,
    ) -> Endpoint0outdataPktAckstallreturnedW<Dev04Spec> {
        Endpoint0outdataPktAckstallreturnedW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 0 OUT data packet NAK returned"]
    #[inline(always)]
    pub fn endpoint0outdata_pkt_nakreturned(
        &mut self,
    ) -> Endpoint0outdataPktNakreturnedW<Dev04Spec> {
        Endpoint0outdataPktNakreturnedW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint 0 IN data packet ACK received or STALL returned"]
    #[inline(always)]
    pub fn endpoint0indata_pkt_ackrxd_or_stallreturned(
        &mut self,
    ) -> Endpoint0indataPktAckrxdOrStallreturnedW<Dev04Spec> {
        Endpoint0indataPktAckrxdOrStallreturnedW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint 0 IN data packet NAK returned"]
    #[inline(always)]
    pub fn endpoint0indata_pkt_nakreturned(&mut self) -> Endpoint0indataPktNakreturnedW<Dev04Spec> {
        Endpoint0indataPktNakreturnedW::new(self, 4)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev04Spec;
impl crate::RegisterSpec for Dev04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev04::R`](R) reader structure"]
impl crate::Readable for Dev04Spec {}
#[doc = "`write(|w| ..)` method takes [`dev04::W`](W) writer structure"]
impl crate::Writable for Dev04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV04 to value 0"]
impl crate::Resettable for Dev04Spec {}
