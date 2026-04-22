#[doc = "Register `SPIF21C` reader"]
pub type R = crate::R<Spif21cSpec>;
#[doc = "Register `SPIF21C` writer"]
pub type W = crate::W<Spif21cSpec>;
#[doc = "Field `ADDRLBND07` reader - ADDR_LBND07"]
pub type Addrlbnd07R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND07` writer - ADDR_LBND07"]
pub type Addrlbnd07W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND07` reader - ADDR_UBND07"]
pub type Addrubnd07R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND07` writer - ADDR_UBND07"]
pub type Addrubnd07W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND07"]
    #[inline(always)]
    pub fn addrlbnd07(&self) -> Addrlbnd07R {
        Addrlbnd07R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND07"]
    #[inline(always)]
    pub fn addrubnd07(&self) -> Addrubnd07R {
        Addrubnd07R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND07"]
    #[inline(always)]
    pub fn addrlbnd07(&mut self) -> Addrlbnd07W<Spif21cSpec> {
        Addrlbnd07W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND07"]
    #[inline(always)]
    pub fn addrubnd07(&mut self) -> Addrubnd07W<Spif21cSpec> {
        Addrubnd07W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif21c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif21c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif21cSpec;
impl crate::RegisterSpec for Spif21cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif21c::R`](R) reader structure"]
impl crate::Readable for Spif21cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif21c::W`](W) writer structure"]
impl crate::Writable for Spif21cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF21C to value 0"]
impl crate::Resettable for Spif21cSpec {}
