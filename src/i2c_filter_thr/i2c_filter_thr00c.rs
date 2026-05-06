#[doc = "Register `I2C_FILTER_THR00C` reader"]
pub type R = crate::R<I2cFilterThr00cSpec>;
#[doc = "Register `I2C_FILTER_THR00C` writer"]
pub type W = crate::W<I2cFilterThr00cSpec>;
#[doc = "Field `CFG` reader - CFG"]
pub type CfgR = crate::FieldReader<u32>;
#[doc = "Field `CFG` writer - CFG"]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CFG"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFG"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CfgW<I2cFilterThr00cSpec> {
        CfgW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr00cSpec;
impl crate::RegisterSpec for I2cFilterThr00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr00c::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr00cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr00c::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR00C to value 0"]
impl crate::Resettable for I2cFilterThr00cSpec {}
