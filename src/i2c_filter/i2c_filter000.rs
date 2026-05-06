#[doc = "Register `I2C_FILTER000` reader"]
pub type R = crate::R<I2cFilter000Spec>;
#[doc = "Register `I2C_FILTER000` writer"]
pub type W = crate::W<I2cFilter000Spec>;
#[doc = "Field `TOPRST` reader - TOP_RST"]
pub type ToprstR = crate::BitReader;
#[doc = "Field `TOPRST` writer - TOP_RST"]
pub type ToprstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TOP_RST"]
    #[inline(always)]
    pub fn toprst(&self) -> ToprstR {
        ToprstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOP_RST"]
    #[inline(always)]
    pub fn toprst(&mut self) -> ToprstW<I2cFilter000Spec> {
        ToprstW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilter000Spec;
impl crate::RegisterSpec for I2cFilter000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter000::R`](R) reader structure"]
impl crate::Readable for I2cFilter000Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter000::W`](W) writer structure"]
impl crate::Writable for I2cFilter000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER000 to value 0"]
impl crate::Resettable for I2cFilter000Spec {}
