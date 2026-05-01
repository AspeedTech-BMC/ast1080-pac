#[doc = "Register `GSRAM0EC` reader"]
pub type R = crate::R<Gsram0ecSpec>;
#[doc = "Register `GSRAM0EC` writer"]
pub type W = crate::W<Gsram0ecSpec>;
#[doc = "Field `WLOCK27` reader - WLOCK27"]
pub type Wlock27R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK27` writer - WLOCK27"]
pub type Wlock27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK27"]
    #[inline(always)]
    pub fn wlock27(&self) -> Wlock27R {
        Wlock27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK27"]
    #[inline(always)]
    pub fn wlock27(&mut self) -> Wlock27W<Gsram0ecSpec> {
        Wlock27W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK27\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0ecSpec;
impl crate::RegisterSpec for Gsram0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0ec::R`](R) reader structure"]
impl crate::Readable for Gsram0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram0ec::W`](W) writer structure"]
impl crate::Writable for Gsram0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0EC to value 0"]
impl crate::Resettable for Gsram0ecSpec {}
