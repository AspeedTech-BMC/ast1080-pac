#[doc = "Register `HCIPIO028` reader"]
pub type R = crate::R<Hcipio028Spec>;
#[doc = "Register `HCIPIO028` writer"]
pub type W = crate::W<Hcipio028Spec>;
#[doc = "Field `REGPIOTXTHLDSIGNALEN` reader - REG_PIO_TX_THLD_SIGNAL_EN"]
pub type RegpiotxthldsignalenR = crate::BitReader;
#[doc = "Field `REGPIOTXTHLDSIGNALEN` writer - REG_PIO_TX_THLD_SIGNAL_EN"]
pub type RegpiotxthldsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIORXTHLDSIGNALEN` reader - REG_PIO_RX_THLD_SIGNAL_EN"]
pub type RegpiorxthldsignalenR = crate::BitReader;
#[doc = "Field `REGPIORXTHLDSIGNALEN` writer - REG_PIO_RX_THLD_SIGNAL_EN"]
pub type RegpiorxthldsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOIBISTATUSTHLDSIGNALEN` reader - REG_PIO_IBI_STATUS_THLD_SIGNAL_EN"]
pub type RegpioibistatusthldsignalenR = crate::BitReader;
#[doc = "Field `REGPIOIBISTATUSTHLDSIGNALEN` writer - REG_PIO_IBI_STATUS_THLD_SIGNAL_EN"]
pub type RegpioibistatusthldsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOCMDQUEUEREADYSIGNALEN` reader - REG_PIO_CMD_QUEUE_READY_SIGNAL_EN"]
pub type RegpiocmdqueuereadysignalenR = crate::BitReader;
#[doc = "Field `REGPIOCMDQUEUEREADYSIGNALEN` writer - REG_PIO_CMD_QUEUE_READY_SIGNAL_EN"]
pub type RegpiocmdqueuereadysignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIORESPREADYSIGNALEN` reader - REG_PIO_RESP_READY_SIGNAL_EN"]
pub type RegpiorespreadysignalenR = crate::BitReader;
#[doc = "Field `REGPIORESPREADYSIGNALEN` writer - REG_PIO_RESP_READY_SIGNAL_EN"]
pub type RegpiorespreadysignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOTRANSFERABORTSIGNALEN` reader - REG_PIO_TRANSFER_ABORT_SIGNAL_EN"]
pub type RegpiotransferabortsignalenR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERABORTSIGNALEN` writer - REG_PIO_TRANSFER_ABORT_SIGNAL_EN"]
pub type RegpiotransferabortsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGPIOTRANSFERERRSIGNALEN` reader - REG_PIO_TRANSFER_ERR_SIGNAL_EN"]
pub type RegpiotransfererrsignalenR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERERRSIGNALEN` writer - REG_PIO_TRANSFER_ERR_SIGNAL_EN"]
pub type RegpiotransfererrsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_PIO_TX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiotxthldsignalen(&self) -> RegpiotxthldsignalenR {
        RegpiotxthldsignalenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_PIO_RX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiorxthldsignalen(&self) -> RegpiorxthldsignalenR {
        RegpiorxthldsignalenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_PIO_IBI_STATUS_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpioibistatusthldsignalen(&self) -> RegpioibistatusthldsignalenR {
        RegpioibistatusthldsignalenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_PIO_CMD_QUEUE_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiocmdqueuereadysignalen(&self) -> RegpiocmdqueuereadysignalenR {
        RegpiocmdqueuereadysignalenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_PIO_RESP_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiorespreadysignalen(&self) -> RegpiorespreadysignalenR {
        RegpiorespreadysignalenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiotransferabortsignalen(&self) -> RegpiotransferabortsignalenR {
        RegpiotransferabortsignalenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiotransfererrsignalen(&self) -> RegpiotransfererrsignalenR {
        RegpiotransfererrsignalenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_PIO_TX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiotxthldsignalen(&mut self) -> RegpiotxthldsignalenW<Hcipio028Spec> {
        RegpiotxthldsignalenW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_PIO_RX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiorxthldsignalen(&mut self) -> RegpiorxthldsignalenW<Hcipio028Spec> {
        RegpiorxthldsignalenW::new(self, 1)
    }
    #[doc = "Bit 2 - REG_PIO_IBI_STATUS_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpioibistatusthldsignalen(&mut self) -> RegpioibistatusthldsignalenW<Hcipio028Spec> {
        RegpioibistatusthldsignalenW::new(self, 2)
    }
    #[doc = "Bit 3 - REG_PIO_CMD_QUEUE_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiocmdqueuereadysignalen(&mut self) -> RegpiocmdqueuereadysignalenW<Hcipio028Spec> {
        RegpiocmdqueuereadysignalenW::new(self, 3)
    }
    #[doc = "Bit 4 - REG_PIO_RESP_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiorespreadysignalen(&mut self) -> RegpiorespreadysignalenW<Hcipio028Spec> {
        RegpiorespreadysignalenW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiotransferabortsignalen(&mut self) -> RegpiotransferabortsignalenW<Hcipio028Spec> {
        RegpiotransferabortsignalenW::new(self, 5)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn regpiotransfererrsignalen(&mut self) -> RegpiotransfererrsignalenW<Hcipio028Spec> {
        RegpiotransfererrsignalenW::new(self, 9)
    }
}
#[doc = "PIO\\_INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio028Spec;
impl crate::RegisterSpec for Hcipio028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio028::R`](R) reader structure"]
impl crate::Readable for Hcipio028Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio028::W`](W) writer structure"]
impl crate::Writable for Hcipio028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO028 to value 0"]
impl crate::Resettable for Hcipio028Spec {}
