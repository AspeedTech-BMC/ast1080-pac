#[doc = "Register `GSRAM0BC` reader"]
pub type R = crate::R<Gsram0bcSpec>;
#[doc = "Register `GSRAM0BC` writer"]
pub type W = crate::W<Gsram0bcSpec>;
#[doc = "Field `WLOCK15` reader - WLOCK15"]
pub type Wlock15R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK15` writer - WLOCK15"]
pub type Wlock15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK15"]
    #[inline(always)]
    pub fn wlock15(&self) -> Wlock15R {
        Wlock15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK15"]
    #[inline(always)]
    pub fn wlock15(&mut self) -> Wlock15W<Gsram0bcSpec> {
        Wlock15W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK15\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0bcSpec;
impl crate::RegisterSpec for Gsram0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0bc::R`](R) reader structure"]
impl crate::Readable for Gsram0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram0bc::W`](W) writer structure"]
impl crate::Writable for Gsram0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0BC to value 0"]
impl crate::Resettable for Gsram0bcSpec {}
