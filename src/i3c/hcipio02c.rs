#[doc = "Register `HCIPIO02C` reader"]
pub type R = crate::R<Hcipio02cSpec>;
#[doc = "Register `HCIPIO02C` writer"]
pub type W = crate::W<Hcipio02cSpec>;
#[doc = "Field `REGPIOTXTHLDFORCE` reader - REG_PIO_TX_THLD_FORCE"]
pub type RegpiotxthldforceR = crate::BitReader;
#[doc = "Field `REGPIOTXTHLDFORCE` writer - REG_PIO_TX_THLD_FORCE"]
pub type RegpiotxthldforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIORXTHLDFORCE` reader - REG_PIO_RX_THLD_FORCE"]
pub type RegpiorxthldforceR = crate::BitReader;
#[doc = "Field `REGPIORXTHLDFORCE` writer - REG_PIO_RX_THLD_FORCE"]
pub type RegpiorxthldforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOIBITHLDFORCE` reader - REG_PIO_IBI_THLD_FORCE"]
pub type RegpioibithldforceR = crate::BitReader;
#[doc = "Field `REGPIOIBITHLDFORCE` writer - REG_PIO_IBI_THLD_FORCE"]
pub type RegpioibithldforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOCMDQUEUEREADYFORCE` reader - REG_PIO_CMD_QUEUE_READY_FORCE"]
pub type RegpiocmdqueuereadyforceR = crate::BitReader;
#[doc = "Field `REGPIOCMDQUEUEREADYFORCE` writer - REG_PIO_CMD_QUEUE_READY_FORCE"]
pub type RegpiocmdqueuereadyforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIORESPREADYFORCE` reader - REG_PIO_RESP_READY_FORCE"]
pub type RegpiorespreadyforceR = crate::BitReader;
#[doc = "Field `REGPIORESPREADYFORCE` writer - REG_PIO_RESP_READY_FORCE"]
pub type RegpiorespreadyforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPIOTRANSFERABORTFORCE` reader - REG_PIO_TRANSFER_ABORT_FORCE"]
pub type RegpiotransferabortforceR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERABORTFORCE` writer - REG_PIO_TRANSFER_ABORT_FORCE"]
pub type RegpiotransferabortforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGPIOTRANSFERERRFORCE` reader - REG_PIO_TRANSFER_ERR_FORCE"]
pub type RegpiotransfererrforceR = crate::BitReader;
#[doc = "Field `REGPIOTRANSFERERRFORCE` writer - REG_PIO_TRANSFER_ERR_FORCE"]
pub type RegpiotransfererrforceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_PIO_TX_THLD_FORCE"]
    #[inline(always)]
    pub fn regpiotxthldforce(&self) -> RegpiotxthldforceR {
        RegpiotxthldforceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_PIO_RX_THLD_FORCE"]
    #[inline(always)]
    pub fn regpiorxthldforce(&self) -> RegpiorxthldforceR {
        RegpiorxthldforceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_PIO_IBI_THLD_FORCE"]
    #[inline(always)]
    pub fn regpioibithldforce(&self) -> RegpioibithldforceR {
        RegpioibithldforceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_PIO_CMD_QUEUE_READY_FORCE"]
    #[inline(always)]
    pub fn regpiocmdqueuereadyforce(&self) -> RegpiocmdqueuereadyforceR {
        RegpiocmdqueuereadyforceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_PIO_RESP_READY_FORCE"]
    #[inline(always)]
    pub fn regpiorespreadyforce(&self) -> RegpiorespreadyforceR {
        RegpiorespreadyforceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_FORCE"]
    #[inline(always)]
    pub fn regpiotransferabortforce(&self) -> RegpiotransferabortforceR {
        RegpiotransferabortforceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_FORCE"]
    #[inline(always)]
    pub fn regpiotransfererrforce(&self) -> RegpiotransfererrforceR {
        RegpiotransfererrforceR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_PIO_TX_THLD_FORCE"]
    #[inline(always)]
    pub fn regpiotxthldforce(&mut self) -> RegpiotxthldforceW<Hcipio02cSpec> {
        RegpiotxthldforceW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_PIO_RX_THLD_FORCE"]
    #[inline(always)]
    pub fn regpiorxthldforce(&mut self) -> RegpiorxthldforceW<Hcipio02cSpec> {
        RegpiorxthldforceW::new(self, 1)
    }
    #[doc = "Bit 2 - REG_PIO_IBI_THLD_FORCE"]
    #[inline(always)]
    pub fn regpioibithldforce(&mut self) -> RegpioibithldforceW<Hcipio02cSpec> {
        RegpioibithldforceW::new(self, 2)
    }
    #[doc = "Bit 3 - REG_PIO_CMD_QUEUE_READY_FORCE"]
    #[inline(always)]
    pub fn regpiocmdqueuereadyforce(&mut self) -> RegpiocmdqueuereadyforceW<Hcipio02cSpec> {
        RegpiocmdqueuereadyforceW::new(self, 3)
    }
    #[doc = "Bit 4 - REG_PIO_RESP_READY_FORCE"]
    #[inline(always)]
    pub fn regpiorespreadyforce(&mut self) -> RegpiorespreadyforceW<Hcipio02cSpec> {
        RegpiorespreadyforceW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_PIO_TRANSFER_ABORT_FORCE"]
    #[inline(always)]
    pub fn regpiotransferabortforce(&mut self) -> RegpiotransferabortforceW<Hcipio02cSpec> {
        RegpiotransferabortforceW::new(self, 5)
    }
    #[doc = "Bit 9 - REG_PIO_TRANSFER_ERR_FORCE"]
    #[inline(always)]
    pub fn regpiotransfererrforce(&mut self) -> RegpiotransfererrforceW<Hcipio02cSpec> {
        RegpiotransfererrforceW::new(self, 9)
    }
}
#[doc = "PIO\\_INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio02cSpec;
impl crate::RegisterSpec for Hcipio02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio02c::R`](R) reader structure"]
impl crate::Readable for Hcipio02cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcipio02c::W`](W) writer structure"]
impl crate::Writable for Hcipio02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO02C to value 0"]
impl crate::Resettable for Hcipio02cSpec {}
