#[doc = "Register `GPIO84C` reader"]
pub type R = crate::R<Gpio84cSpec>;
#[doc = "Register `GPIO84C` writer"]
pub type W = crate::W<Gpio84cSpec>;
#[doc = "Field `GPIO060WrPrivilegeOfMaster` reader - GPIO060 Write Privilege of Master"]
pub type Gpio060wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO060WrPrivilegeOfMaster` writer - GPIO060 Write Privilege of Master"]
pub type Gpio060wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO061WrPrivilegeOfMaster` reader - GPIO061 Write Privilege of Master"]
pub type Gpio061wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO061WrPrivilegeOfMaster` writer - GPIO061 Write Privilege of Master"]
pub type Gpio061wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO062WrPrivilegeOfMaster` reader - GPIO062 Write Privilege of Master"]
pub type Gpio062wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO062WrPrivilegeOfMaster` writer - GPIO062 Write Privilege of Master"]
pub type Gpio062wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO063WrPrivilegeOfMaster` reader - GPIO063 Write Privilege of Master"]
pub type Gpio063wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO063WrPrivilegeOfMaster` writer - GPIO063 Write Privilege of Master"]
pub type Gpio063wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO060 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio060wr_privilege_of_master(&self) -> Gpio060wrPrivilegeOfMasterR {
        Gpio060wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO061 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio061wr_privilege_of_master(&self) -> Gpio061wrPrivilegeOfMasterR {
        Gpio061wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO062 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio062wr_privilege_of_master(&self) -> Gpio062wrPrivilegeOfMasterR {
        Gpio062wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO063 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio063wr_privilege_of_master(&self) -> Gpio063wrPrivilegeOfMasterR {
        Gpio063wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO060 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio060wr_privilege_of_master(&mut self) -> Gpio060wrPrivilegeOfMasterW<Gpio84cSpec> {
        Gpio060wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO061 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio061wr_privilege_of_master(&mut self) -> Gpio061wrPrivilegeOfMasterW<Gpio84cSpec> {
        Gpio061wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO062 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio062wr_privilege_of_master(&mut self) -> Gpio062wrPrivilegeOfMasterW<Gpio84cSpec> {
        Gpio062wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO063 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio063wr_privilege_of_master(&mut self) -> Gpio063wrPrivilegeOfMasterW<Gpio84cSpec> {
        Gpio063wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio84c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio84c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio84cSpec;
impl crate::RegisterSpec for Gpio84cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio84c::R`](R) reader structure"]
impl crate::Readable for Gpio84cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio84c::W`](W) writer structure"]
impl crate::Writable for Gpio84cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO84C to value 0xffff_ffff"]
impl crate::Resettable for Gpio84cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
