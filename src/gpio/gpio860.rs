#[doc = "Register `GPIO860` reader"]
pub type R = crate::R<Gpio860Spec>;
#[doc = "Register `GPIO860` writer"]
pub type W = crate::W<Gpio860Spec>;
#[doc = "Field `GPIO080WrPrivilegeOfMaster` reader - GPIO080 Write Privilege of Master"]
pub type Gpio080wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO080WrPrivilegeOfMaster` writer - GPIO080 Write Privilege of Master"]
pub type Gpio080wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO081WrPrivilegeOfMaster` reader - GPIO081 Write Privilege of Master"]
pub type Gpio081wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO081WrPrivilegeOfMaster` writer - GPIO081 Write Privilege of Master"]
pub type Gpio081wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO082WrPrivilegeOfMaster` reader - GPIO082 Write Privilege of Master"]
pub type Gpio082wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO082WrPrivilegeOfMaster` writer - GPIO082 Write Privilege of Master"]
pub type Gpio082wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO083WrPrivilegeOfMaster` reader - GPIO083 Write Privilege of Master"]
pub type Gpio083wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO083WrPrivilegeOfMaster` writer - GPIO083 Write Privilege of Master"]
pub type Gpio083wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO080 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio080wr_privilege_of_master(&self) -> Gpio080wrPrivilegeOfMasterR {
        Gpio080wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO081 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio081wr_privilege_of_master(&self) -> Gpio081wrPrivilegeOfMasterR {
        Gpio081wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO082 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio082wr_privilege_of_master(&self) -> Gpio082wrPrivilegeOfMasterR {
        Gpio082wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO083 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio083wr_privilege_of_master(&self) -> Gpio083wrPrivilegeOfMasterR {
        Gpio083wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO080 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio080wr_privilege_of_master(&mut self) -> Gpio080wrPrivilegeOfMasterW<Gpio860Spec> {
        Gpio080wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO081 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio081wr_privilege_of_master(&mut self) -> Gpio081wrPrivilegeOfMasterW<Gpio860Spec> {
        Gpio081wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO082 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio082wr_privilege_of_master(&mut self) -> Gpio082wrPrivilegeOfMasterW<Gpio860Spec> {
        Gpio082wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO083 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio083wr_privilege_of_master(&mut self) -> Gpio083wrPrivilegeOfMasterW<Gpio860Spec> {
        Gpio083wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio860::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio860::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio860Spec;
impl crate::RegisterSpec for Gpio860Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio860::R`](R) reader structure"]
impl crate::Readable for Gpio860Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio860::W`](W) writer structure"]
impl crate::Writable for Gpio860Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO860 to value 0xffff_ffff"]
impl crate::Resettable for Gpio860Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
