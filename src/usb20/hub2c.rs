#[doc = "Register `HUB2C` reader"]
pub type R = crate::R<Hub2cSpec>;
#[doc = "Register `HUB2C` writer"]
pub type W = crate::W<Hub2cSpec>;
#[doc = "Field `IsochronousINFailureCounter` reader - Isochronous IN Failure Counter"]
pub type IsochronousInfailureCounterR = crate::FieldReader<u16>;
#[doc = "Field `IsochronousINFailureCounter` writer - Isochronous IN Failure Counter"]
pub type IsochronousInfailureCounterW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `IsochronousOUTFailureCounter` reader - Isochronous OUT Failure Counter"]
pub type IsochronousOutfailureCounterR = crate::FieldReader<u16>;
#[doc = "Field `IsochronousOUTFailureCounter` writer - Isochronous OUT Failure Counter"]
pub type IsochronousOutfailureCounterW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - Isochronous IN Failure Counter"]
    #[inline(always)]
    pub fn isochronous_infailure_counter(&self) -> IsochronousInfailureCounterR {
        IsochronousInfailureCounterR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Isochronous OUT Failure Counter"]
    #[inline(always)]
    pub fn isochronous_outfailure_counter(&self) -> IsochronousOutfailureCounterR {
        IsochronousOutfailureCounterR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Isochronous IN Failure Counter"]
    #[inline(always)]
    pub fn isochronous_infailure_counter(&mut self) -> IsochronousInfailureCounterW<Hub2cSpec> {
        IsochronousInfailureCounterW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Isochronous OUT Failure Counter"]
    #[inline(always)]
    pub fn isochronous_outfailure_counter(&mut self) -> IsochronousOutfailureCounterW<Hub2cSpec> {
        IsochronousOutfailureCounterW::new(self, 16)
    }
}
#[doc = "Isochronous Transaction Fail Accumulator \\regdebugh\n\nYou can [`read`](crate::Reg::read) this register and get [`hub2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub2cSpec;
impl crate::RegisterSpec for Hub2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub2c::R`](R) reader structure"]
impl crate::Readable for Hub2cSpec {}
#[doc = "`write(|w| ..)` method takes [`hub2c::W`](W) writer structure"]
impl crate::Writable for Hub2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB2C to value 0"]
impl crate::Resettable for Hub2cSpec {}
