#[doc = "Register `I2CGLOBAL01C` reader"]
pub type R = crate::R<I2cglobal01cSpec>;
#[doc = "Register `I2CGLOBAL01C` writer"]
pub type W = crate::W<I2cglobal01cSpec>;
#[doc = "Field `XARBCFG` reader - XARB_CFG"]
pub type XarbcfgR = crate::FieldReader<u32>;
#[doc = "Field `XARBCFG` writer - XARB_CFG"]
pub type XarbcfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - XARB_CFG"]
    #[inline(always)]
    pub fn xarbcfg(&self) -> XarbcfgR {
        XarbcfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XARB_CFG"]
    #[inline(always)]
    pub fn xarbcfg(&mut self) -> XarbcfgW<I2cglobal01cSpec> {
        XarbcfgW::new(self, 0)
    }
}
#[doc = "I2CG\\_MARB\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cglobal01cSpec;
impl crate::RegisterSpec for I2cglobal01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cglobal01c::R`](R) reader structure"]
impl crate::Readable for I2cglobal01cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cglobal01c::W`](W) writer structure"]
impl crate::Writable for I2cglobal01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CGLOBAL01C to value 0"]
impl crate::Resettable for I2cglobal01cSpec {}
