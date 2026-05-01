#[doc = "Register `SCUF5C` reader"]
pub type R = crate::R<Scuf5cSpec>;
#[doc = "Register `SCUF5C` writer"]
pub type W = crate::W<Scuf5cSpec>;
#[doc = "Field `SCUREGRSTB80` reader - SCU_REG_RST_B80"]
pub type Scuregrstb80R = crate::BitReader;
#[doc = "Field `SCUREGRSTB80` writer - SCU_REG_RST_B80"]
pub type Scuregrstb80W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_B80"]
    #[inline(always)]
    pub fn scuregrstb80(&self) -> Scuregrstb80R {
        Scuregrstb80R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_B80"]
    #[inline(always)]
    pub fn scuregrstb80(&mut self) -> Scuregrstb80W<Scuf5cSpec> {
        Scuregrstb80W::new(self, 0)
    }
}
#[doc = "Reset Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf5cSpec;
impl crate::RegisterSpec for Scuf5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf5c::R`](R) reader structure"]
impl crate::Readable for Scuf5cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf5c::W`](W) writer structure"]
impl crate::Writable for Scuf5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF5C to value 0"]
impl crate::Resettable for Scuf5cSpec {}
