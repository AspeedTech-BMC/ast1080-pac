#[doc = "Register `I2C_FILTER_THR010` reader"]
pub type R = crate::R<I2cFilterThr010Spec>;
#[doc = "Register `I2C_FILTER_THR010` writer"]
pub type W = crate::W<I2cFilterThr010Spec>;
#[doc = "Field `TMRCFG` reader - TMR_CFG"]
pub type TmrcfgR = crate::FieldReader<u32>;
#[doc = "Field `TMRCFG` writer - TMR_CFG"]
pub type TmrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TMR_CFG"]
    #[inline(always)]
    pub fn tmrcfg(&self) -> TmrcfgR {
        TmrcfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TMR_CFG"]
    #[inline(always)]
    pub fn tmrcfg(&mut self) -> TmrcfgW<I2cFilterThr010Spec> {
        TmrcfgW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_TMR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr010Spec;
impl crate::RegisterSpec for I2cFilterThr010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr010::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr010Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr010::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR010 to value 0"]
impl crate::Resettable for I2cFilterThr010Spec {}
