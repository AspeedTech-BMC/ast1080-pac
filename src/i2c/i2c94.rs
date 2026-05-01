#[doc = "Register `I2C94` reader"]
pub type R = crate::R<I2c94Spec>;
#[doc = "Register `I2C94` writer"]
pub type W = crate::W<I2c94Spec>;
#[doc = "Field `FUNCCFG` reader - FUNC_CFG"]
pub type FunccfgR = crate::FieldReader<u32>;
#[doc = "Field `FUNCCFG` writer - FUNC_CFG"]
pub type FunccfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FUNC_CFG"]
    #[inline(always)]
    pub fn funccfg(&self) -> FunccfgR {
        FunccfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FUNC_CFG"]
    #[inline(always)]
    pub fn funccfg(&mut self) -> FunccfgW<I2c94Spec> {
        FunccfgW::new(self, 0)
    }
}
#[doc = "I2CC\\_VERSION\\_CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c94::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c94::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c94Spec;
impl crate::RegisterSpec for I2c94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c94::R`](R) reader structure"]
impl crate::Readable for I2c94Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c94::W`](W) writer structure"]
impl crate::Writable for I2c94Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C94 to value 0xffff_ffff"]
impl crate::Resettable for I2c94Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
