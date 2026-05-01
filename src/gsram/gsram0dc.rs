#[doc = "Register `GSRAM0DC` reader"]
pub type R = crate::R<Gsram0dcSpec>;
#[doc = "Register `GSRAM0DC` writer"]
pub type W = crate::W<Gsram0dcSpec>;
#[doc = "Field `WLOCK23` reader - WLOCK23"]
pub type Wlock23R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK23` writer - WLOCK23"]
pub type Wlock23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK23"]
    #[inline(always)]
    pub fn wlock23(&self) -> Wlock23R {
        Wlock23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK23"]
    #[inline(always)]
    pub fn wlock23(&mut self) -> Wlock23W<Gsram0dcSpec> {
        Wlock23W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK23\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0dcSpec;
impl crate::RegisterSpec for Gsram0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0dc::R`](R) reader structure"]
impl crate::Readable for Gsram0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram0dc::W`](W) writer structure"]
impl crate::Writable for Gsram0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0DC to value 0"]
impl crate::Resettable for Gsram0dcSpec {}
