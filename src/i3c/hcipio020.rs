#[doc = "Register `HCIPIO020` reader"]
pub type R = crate::R<Hcipio020Spec>;
#[doc = "Register `HCIPIO020` writer"]
pub type W = crate::W<Hcipio020Spec>;
#[doc = "Field `REGPIOTXTHLDSTAT` reader - REG_PIO_TX_THLD_STAT"]
pub type RegpiotxthldstatR = crate::BitReader;
#[doc = "Field `REGPIORXTHLDSTAT` reader - REG_PIO_RX_THLD_STAT"]
pub type RegpiorxthldstatR = crate::BitReader;
#[doc = "Field `REGPIOIBISTATUSTHLDSTAT` reader - REG_PIO_IBI_STATUS_THLD_STAT"]
pub type RegpioibistatusthldstatR = crate::BitReader;
#[doc = "Field `REGPIOCMDQUEUEREADYSTAT` reader - REG_PIO_CMD_QUEUE_READY_STAT"]
pub type RegpiocmdqueuereadystatR = crate::BitReader;
#[doc = "Field `REGPIORESPREADYSTAT` reader - REG_PIO_RESP_READY_STAT"]
pub type RegpiorespreadystatR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERABORTSTAT` reader - REG_PIO_TRANSFER_ABORT_STAT"]
pub type RegpiotransferabortstatR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERABORTSTAT` writer - REG_PIO_TRANSFER_ABORT_STAT"]
pub type RegpiotransferabortstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGPIOTRANSFERERRSTAT` reader - REG_PIO_TRANSFER_ERR_STAT"]
pub type RegpiotransfererrstatR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERERRSTAT` writer - REG_PIO_TRANSFER_ERR_STAT"]
pub type RegpiotransfererrstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_PIO_TX_THLD_STAT"]
    #[inline(always)]
    pub fn regpiotxthldstat(&self) -> RegpiotxthldstatR {
        RegpiotxthldstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_PIO_RX_THLD_STAT"]
    #[inline(always)]
    pub fn regpiorxthldstat(&self) -> RegpiorxthldstatR {
        RegpiorxthldstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_PIO_IBI_STATUS_THLD_STAT"]
    #[inline(always)]
    pub fn regpioibistatusthldstat(&self) -> RegpioibistatusthldstatR {
        RegpioibistatusthldstatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_PIO_CMD_QUEUE_READY_STAT"]
    #[inline(always)]
    pub fn regpiocmdqueuereadystat(&self) -> RegpiocmdqueuereadystatR {
        RegpiocmdqueuereadystatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_PIO_RESP_READY_STAT"]
    #[inline(always)]
    pub fn regpiorespreadystat(&self) -> RegpiorespreadystatR {
        RegpiorespreadystatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_STAT"]
    #[inline(always)]
    pub fn regpiotransferabortstat(&self) -> RegpiotransferabortstatR {
        RegpiotransferabortstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_STAT"]
    #[inline(always)]
    pub fn regpiotransfererrstat(&self) -> RegpiotransfererrstatR {
        RegpiotransfererrstatR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_STAT"]
    #[inline(always)]
    pub fn regpiotransferabortstat(&mut self) -> RegpiotransferabortstatW<Hcipio020Spec> {
        RegpiotransferabortstatW::new(self, 5)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_STAT"]
    #[inline(always)]
    pub fn regpiotransfererrstat(&mut self) -> RegpiotransfererrstatW<Hcipio020Spec> {
        RegpiotransfererrstatW::new(self, 9)
    }
}
#[doc = "PIO\\_INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio020Spec;
impl crate::RegisterSpec for Hcipio020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio020::R`](R) reader structure"]
impl crate::Readable for Hcipio020Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio020::W`](W) writer structure"]
impl crate::Writable for Hcipio020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO020 to value 0"]
impl crate::Resettable for Hcipio020Spec {}
