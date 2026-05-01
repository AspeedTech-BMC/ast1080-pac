#[doc = "Register `SCUF6C` reader"]
pub type R = crate::R<Scuf6cSpec>;
#[doc = "Register `SCUF6C` writer"]
pub type W = crate::W<Scuf6cSpec>;
#[doc = "Field `SCUREGRSTDC4` reader - SCU_REG_RST_DC4"]
pub type Scuregrstdc4R = crate::BitReader;
#[doc = "Field `SCUREGRSTDC4` writer - SCU_REG_RST_DC4"]
pub type Scuregrstdc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRSTDC8` reader - SCU_REG_RST_DC8"]
pub type Scuregrstdc8R = crate::BitReader;
#[doc = "Field `SCUREGRSTDC8` writer - SCU_REG_RST_DC8"]
pub type Scuregrstdc8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - SCU_REG_RST_DC4"]
    #[inline(always)]
    pub fn scuregrstdc4(&self) -> Scuregrstdc4R {
        Scuregrstdc4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_RST_DC8"]
    #[inline(always)]
    pub fn scuregrstdc8(&self) -> Scuregrstdc8R {
        Scuregrstdc8R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - SCU_REG_RST_DC4"]
    #[inline(always)]
    pub fn scuregrstdc4(&mut self) -> Scuregrstdc4W<Scuf6cSpec> {
        Scuregrstdc4W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_RST_DC8"]
    #[inline(always)]
    pub fn scuregrstdc8(&mut self) -> Scuregrstdc8W<Scuf6cSpec> {
        Scuregrstdc8W::new(self, 18)
    }
}
#[doc = "Reset Control 28 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf6cSpec;
impl crate::RegisterSpec for Scuf6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf6c::R`](R) reader structure"]
impl crate::Readable for Scuf6cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf6c::W`](W) writer structure"]
impl crate::Writable for Scuf6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF6C to value 0"]
impl crate::Resettable for Scuf6cSpec {}
