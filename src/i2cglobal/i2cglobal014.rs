#[doc = "Register `I2CGLOBAL014` reader"]
pub type R = crate::R<I2cglobal014Spec>;
#[doc = "Register `I2CGLOBAL014` writer"]
pub type W = crate::W<I2cglobal014Spec>;
#[doc = "Field `FIFOCFG0` reader - FIFO_CFG0"]
pub type Fifocfg0R = crate::FieldReader<u32>;
#[doc = "Field `FIFOCFG0` writer - FIFO_CFG0"]
pub type Fifocfg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO_CFG0"]
    #[inline(always)]
    pub fn fifocfg0(&self) -> Fifocfg0R {
        Fifocfg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO_CFG0"]
    #[inline(always)]
    pub fn fifocfg0(&mut self) -> Fifocfg0W<I2cglobal014Spec> {
        Fifocfg0W::new(self, 0)
    }
}
#[doc = "I2CG\\_FIFO\\_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cglobal014Spec;
impl crate::RegisterSpec for I2cglobal014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cglobal014::R`](R) reader structure"]
impl crate::Readable for I2cglobal014Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cglobal014::W`](W) writer structure"]
impl crate::Writable for I2cglobal014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CGLOBAL014 to value 0x6666_6666"]
impl crate::Resettable for I2cglobal014Spec {
    const RESET_VALUE: u32 = 0x6666_6666;
}
