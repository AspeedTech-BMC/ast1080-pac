#[doc = "Register `HCIPIO024` reader"]
pub type R = crate::R<Hcipio024Spec>;
#[doc = "Register `HCIPIO024` writer"]
pub type W = crate::W<Hcipio024Spec>;
#[doc = "Field `REGPIOTXTHLDSTATEN` reader - REG_PIO_TX_THLD_STAT_EN"]
pub type RegpiotxthldstatenR = crate::BitReader;
#[doc = "Field `REGPIOTXTHLDSTATEN` writer - REG_PIO_TX_THLD_STAT_EN"]
pub type RegpiotxthldstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIORXTHLDSTATEN` reader - REG_PIO_RX_THLD_STAT_EN"]
pub type RegpiorxthldstatenR = crate::BitReader;
#[doc = "Field `REGPIORXTHLDSTATEN` writer - REG_PIO_RX_THLD_STAT_EN"]
pub type RegpiorxthldstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOIBISTATUSTHLDSTATEN` reader - REG_PIO_IBI_STATUS_THLD_STAT_EN"]
pub type RegpioibistatusthldstatenR = crate::BitReader;
#[doc = "Field `REGPIOIBISTATUSTHLDSTATEN` writer - REG_PIO_IBI_STATUS_THLD_STAT_EN"]
pub type RegpioibistatusthldstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOCMDQUEUEREADYSTATEN` reader - REG_PIO_CMD_QUEUE_READY_STAT_EN"]
pub type RegpiocmdqueuereadystatenR = crate::BitReader;
#[doc = "Field `REGPIOCMDQUEUEREADYSTATEN` writer - REG_PIO_CMD_QUEUE_READY_STAT_EN"]
pub type RegpiocmdqueuereadystatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIORESPREADYSTATEN` reader - REG_PIO_RESP_READY_STAT_EN"]
pub type RegpiorespreadystatenR = crate::BitReader;
#[doc = "Field `REGPIORESPREADYSTATEN` writer - REG_PIO_RESP_READY_STAT_EN"]
pub type RegpiorespreadystatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOTRANSFERABORTSTATEN` reader - REG_PIO_TRANSFER_ABORT_STAT_EN"]
pub type RegpiotransferabortstatenR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERABORTSTATEN` writer - REG_PIO_TRANSFER_ABORT_STAT_EN"]
pub type RegpiotransferabortstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGPIOTRANSFERERRSTATEN` reader - REG_PIO_TRANSFER_ERR_STAT_EN"]
pub type RegpiotransfererrstatenR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERERRSTATEN` writer - REG_PIO_TRANSFER_ERR_STAT_EN"]
pub type RegpiotransfererrstatenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_PIO_TX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn regpiotxthldstaten(&self) -> RegpiotxthldstatenR {
        RegpiotxthldstatenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_PIO_RX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn regpiorxthldstaten(&self) -> RegpiorxthldstatenR {
        RegpiorxthldstatenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_PIO_IBI_STATUS_THLD_STAT_EN"]
    #[inline(always)]
    pub fn regpioibistatusthldstaten(&self) -> RegpioibistatusthldstatenR {
        RegpioibistatusthldstatenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_PIO_CMD_QUEUE_READY_STAT_EN"]
    #[inline(always)]
    pub fn regpiocmdqueuereadystaten(&self) -> RegpiocmdqueuereadystatenR {
        RegpiocmdqueuereadystatenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_PIO_RESP_READY_STAT_EN"]
    #[inline(always)]
    pub fn regpiorespreadystaten(&self) -> RegpiorespreadystatenR {
        RegpiorespreadystatenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_STAT_EN"]
    #[inline(always)]
    pub fn regpiotransferabortstaten(&self) -> RegpiotransferabortstatenR {
        RegpiotransferabortstatenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_STAT_EN"]
    #[inline(always)]
    pub fn regpiotransfererrstaten(&self) -> RegpiotransfererrstatenR {
        RegpiotransfererrstatenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_PIO_TX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn regpiotxthldstaten(&mut self) -> RegpiotxthldstatenW<Hcipio024Spec> {
        RegpiotxthldstatenW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_PIO_RX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn regpiorxthldstaten(&mut self) -> RegpiorxthldstatenW<Hcipio024Spec> {
        RegpiorxthldstatenW::new(self, 1)
    }
    #[doc = "Bit 2 - REG_PIO_IBI_STATUS_THLD_STAT_EN"]
    #[inline(always)]
    pub fn regpioibistatusthldstaten(&mut self) -> RegpioibistatusthldstatenW<Hcipio024Spec> {
        RegpioibistatusthldstatenW::new(self, 2)
    }
    #[doc = "Bit 3 - REG_PIO_CMD_QUEUE_READY_STAT_EN"]
    #[inline(always)]
    pub fn regpiocmdqueuereadystaten(&mut self) -> RegpiocmdqueuereadystatenW<Hcipio024Spec> {
        RegpiocmdqueuereadystatenW::new(self, 3)
    }
    #[doc = "Bit 4 - REG_PIO_RESP_READY_STAT_EN"]
    #[inline(always)]
    pub fn regpiorespreadystaten(&mut self) -> RegpiorespreadystatenW<Hcipio024Spec> {
        RegpiorespreadystatenW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_STAT_EN"]
    #[inline(always)]
    pub fn regpiotransferabortstaten(&mut self) -> RegpiotransferabortstatenW<Hcipio024Spec> {
        RegpiotransferabortstatenW::new(self, 5)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_STAT_EN"]
    #[inline(always)]
    pub fn regpiotransfererrstaten(&mut self) -> RegpiotransfererrstatenW<Hcipio024Spec> {
        RegpiotransfererrstatenW::new(self, 9)
    }
}
#[doc = "PIO\\_INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio024Spec;
impl crate::RegisterSpec for Hcipio024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio024::R`](R) reader structure"]
impl crate::Readable for Hcipio024Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio024::W`](W) writer structure"]
impl crate::Writable for Hcipio024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO024 to value 0"]
impl crate::Resettable for Hcipio024Spec {}
