#[doc = "Register `GPIO87C` reader"]
pub type R = crate::R<Gpio87cSpec>;
#[doc = "Register `GPIO87C` writer"]
pub type W = crate::W<Gpio87cSpec>;
#[doc = "Field `GPIO108WrPrivilegeOfMaster` reader - GPIO108 Write Privilege of Master"]
pub type Gpio108wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO108WrPrivilegeOfMaster` writer - GPIO108 Write Privilege of Master"]
pub type Gpio108wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO109WrPrivilegeOfMaster` reader - GPIO109 Write Privilege of Master"]
pub type Gpio109wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO109WrPrivilegeOfMaster` writer - GPIO109 Write Privilege of Master"]
pub type Gpio109wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO110WrPrivilegeOfMaster` reader - GPIO110 Write Privilege of Master"]
pub type Gpio110wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO110WrPrivilegeOfMaster` writer - GPIO110 Write Privilege of Master"]
pub type Gpio110wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO111WrPrivilegeOfMaster` reader - GPIO111 Write Privilege of Master"]
pub type Gpio111wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO111WrPrivilegeOfMaster` writer - GPIO111 Write Privilege of Master"]
pub type Gpio111wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO108 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio108wr_privilege_of_master(&self) -> Gpio108wrPrivilegeOfMasterR {
        Gpio108wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO109 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio109wr_privilege_of_master(&self) -> Gpio109wrPrivilegeOfMasterR {
        Gpio109wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO110 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio110wr_privilege_of_master(&self) -> Gpio110wrPrivilegeOfMasterR {
        Gpio110wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO111 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio111wr_privilege_of_master(&self) -> Gpio111wrPrivilegeOfMasterR {
        Gpio111wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO108 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio108wr_privilege_of_master(&mut self) -> Gpio108wrPrivilegeOfMasterW<Gpio87cSpec> {
        Gpio108wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO109 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio109wr_privilege_of_master(&mut self) -> Gpio109wrPrivilegeOfMasterW<Gpio87cSpec> {
        Gpio109wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO110 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio110wr_privilege_of_master(&mut self) -> Gpio110wrPrivilegeOfMasterW<Gpio87cSpec> {
        Gpio110wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO111 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio111wr_privilege_of_master(&mut self) -> Gpio111wrPrivilegeOfMasterW<Gpio87cSpec> {
        Gpio111wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio87c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio87c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio87cSpec;
impl crate::RegisterSpec for Gpio87cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio87c::R`](R) reader structure"]
impl crate::Readable for Gpio87cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio87c::W`](W) writer structure"]
impl crate::Writable for Gpio87cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO87C to value 0xffff_ffff"]
impl crate::Resettable for Gpio87cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
